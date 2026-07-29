use crate::{
    model::{
        ExecutableInfo, ExecutableOrigin, ExecutableSelectionKind, Finding, PathClass, Profile,
        Report, RuntimeKind, Severity, ToolchainState,
    },
    rules::RuleDefinition,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Language {
    #[default]
    English,
    Japanese,
}

#[must_use]
pub const fn toolchain_state_label(state: ToolchainState, language: Language) -> &'static str {
    match (state, language) {
        (ToolchainState::NotFound, Language::English) => "Not found",
        (ToolchainState::NotFound, Language::Japanese) => "見つかりません",
        (ToolchainState::CandidatesUnconfirmed, Language::English) => {
            "Candidates found / selection unconfirmed"
        }
        (ToolchainState::CandidatesUnconfirmed, Language::Japanese) => "候補あり／選択未確定",
        (ToolchainState::Selected, Language::English) => "Selected",
        (ToolchainState::Selected, Language::Japanese) => "選択済み",
        (ToolchainState::ProbeFailed, Language::English) => "Probe failed",
        (ToolchainState::ProbeFailed, Language::Japanese) => "調査失敗",
    }
}

#[must_use]
pub fn selection_kind_label(kind: ExecutableSelectionKind, language: Language) -> &'static str {
    if language == Language::English {
        return kind.label();
    }
    match kind {
        ExecutableSelectionKind::Alias => "エイリアス",
        ExecutableSelectionKind::Function => "関数",
        ExecutableSelectionKind::Cmdlet => "コマンドレット",
        ExecutableSelectionKind::Builtin => "組み込み",
        ExecutableSelectionKind::ExternalScript => "外部スクリプト",
        ExecutableSelectionKind::Application => "アプリケーション",
    }
}

#[must_use]
pub const fn severity_label(severity: Severity, language: Language) -> &'static str {
    match (severity, language) {
        (Severity::Info, Language::English) => "info",
        (Severity::Info, Language::Japanese) => "情報",
        (Severity::Warning, Language::English) => "warning",
        (Severity::Warning, Language::Japanese) => "警告",
        (Severity::Error, Language::English) => "error",
        (Severity::Error, Language::Japanese) => "エラー",
    }
}

impl Language {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
        }
    }

    #[must_use]
    pub const fn text<'a>(self, english: &'a str, japanese: &'a str) -> &'a str {
        match self {
            Self::English => english,
            Self::Japanese => japanese,
        }
    }
}

#[must_use]
pub fn localize_report(report: &Report, language: Language) -> Report {
    if language == Language::English {
        return report.clone();
    }

    let mut localized = report.clone();
    for executable in &mut localized.executables {
        executable.selection_reason = japanese_selection_reason(executable);
    }
    localized.findings = report
        .findings
        .iter()
        .map(|finding| japanese_finding(report, finding))
        .collect();
    localized
}

fn japanese_selection_reason(executable: &ExecutableInfo) -> String {
    match executable.selection_state {
        ToolchainState::Selected
            if executable.resolution_method
                == crate::model::ExecutableResolutionMethod::PathFallback =>
        {
            "一般的なPATH順で、最初に確認できた実行ファイルを選択しました。".to_owned()
        }
        ToolchainState::Selected => format!(
            "完全な{}セッション情報からコマンドの優先順位を確定しました。",
            executable.resolution_shell.as_deref().unwrap_or("シェル")
        ),
        ToolchainState::CandidatesUnconfirmed
            if executable.shell_session_complete == Some(false) =>
        {
            format!(
                "外部候補は見つかりましたが、親{}のエイリアス、関数、組み込みコマンドを取得できていません。",
                executable.resolution_shell.as_deref().unwrap_or("シェル")
            )
        }
        ToolchainState::CandidatesUnconfirmed => {
            "候補は見つかりましたが、選択を確定するための観測情報が不足しています。".to_owned()
        }
        ToolchainState::NotFound => {
            "実行ファイル候補またはシェルのコマンド定義が見つかりませんでした。".to_owned()
        }
        ToolchainState::ProbeFailed => {
            "信頼できる選択を確定する前に候補の調査が失敗しました。".to_owned()
        }
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "the exhaustive rule-ID translation table is clearer when kept together"
)]
fn japanese_finding(report: &Report, finding: &Finding) -> Finding {
    let mut localized = finding.clone();
    let role = role_from_finding(finding).unwrap_or("ツール");
    match finding.id.as_str() {
        "ENV001" => {
            "ターミナルとエージェントが異なるOS層を使用しています".clone_into(&mut localized.title);
            localized.summary = format!(
                "現在のターミナル層は{}、観測されたエージェントの実行層は{}です。",
                runtime_label(report.runtime.terminal_layer),
                runtime_label(report.agent.runtime)
            );
            localized.suggested_actions = vec![
                "設定を変更する前に、Git、Node、シェル、プロジェクトパスの解決結果を比較してください。".to_owned(),
                "構造化レポートで、両者の関係を示す根拠を確認してください。".to_owned(),
            ];
            localized.verification_steps = vec![
                "表示中のターミナルとエージェント実行環境の両方でToolchainの確認コマンドを実行し、ExecLocusを再診断してください。".to_owned(),
            ];
        }
        "ENV002" => {
            localized.title = format!("WSL実行でWindows版{role}が選択されています");
            localized.summary = format!(
                "ExecLocusはWSL内で動作していますが、{role}はWindows実行ファイルとして解決されています。"
            );
            localized.suggested_actions = vec![
                "Windowsとの相互運用を意図している場合は、この構成を維持できます。".to_owned(),
                format!(
                    "Linux動作を意図する場合は、Linuxネイティブの{role}を導入して優先してください。"
                ),
            ];
            localized.verification_steps = vec![format!(
                "同じシェルで`{}`を実行してから、ExecLocusを再診断してください。",
                verification_for_role(report, role)
            )];
        }
        "ENV003" => {
            let product = product_from_finding(finding);
            localized.title = format!("{product}がWindowsとWSLの両方にあります");
            localized.summary = format!(
                "確実な実行ファイル根拠により、{product}の候補がWindows層とWSL層の両方で見つかりました。"
            );
            localized.suggested_actions = vec![
                "どちらかを変更する前に、バージョンと解決パスを比較してください。".to_owned(),
                "両方の作業方法を意図している場合は、両方の導入を維持できます。".to_owned(),
                "実際に使う作業方法を確認してから、一方の削除または優先度変更を検討してください。"
                    .to_owned(),
            ];
            localized.verification_steps = vec![format!(
                "WindowsとWSLの両方で{product}のToolchain確認コマンドを実行し、ExecLocusを再診断してください。"
            )];
        }
        "ENV004" => {
            let product = product_from_finding(finding);
            localized.title = format!("{product}の設定がOS層をまたいでいます");
            "エージェントの実行層と、主設定または状態の保存層が異なります。"
                .clone_into(&mut localized.summary);
            localized.suggested_actions = vec![
                "可能であれば、書き込み可能なデータベース、キャッシュ、主設定を実行側と同じOS層に置いてください。".to_owned(),
                "エージェント提供元が移植可能と明記した設定ファイルだけを共有してください。".to_owned(),
                "状態を手動で移動する前にバックアップしてください。".to_owned(),
            ];
            localized.verification_steps = vec![
                "設定場所を変更した後にExecLocusを再診断し、ENV004が表示されないことを確認してください。".to_owned(),
            ];
        }
        "FS001" => {
            "WSLプロジェクトがWindowsマウント上にあります".clone_into(&mut localized.title);
            "プロジェクトはWSLからマウントされたWindowsファイルシステム上にあります。相互運用に有効な構成ですが、ファイルシステム特性に差があります。"
                .clone_into(&mut localized.summary);
            localized.suggested_actions = match report.profile {
                Profile::ShareFirst => vec![
                    "Windowsアプリ、エクスプローラー、Coworkからの利用を優先する場合は、この配置を維持してください。".to_owned(),
                    "性能上の理由で移動する前に、影響する処理を実測してください。".to_owned(),
                ],
                Profile::Balanced => vec![
                    "相互運用が役立つ場合は、共有ソースをWindowsマウント上に維持してください。".to_owned(),
                    "対応可能なら、依存キャッシュとビルド出力をWSLネイティブ領域へ置いてください。".to_owned(),
                ],
                Profile::LinuxFirst => vec![
                    r"WSLネイティブの作業コピーを検討し、Windowsからは\\wsl.localhost経由で参照してください。".to_owned(),
                    "移動前に、必要なWindowsアプリがWSL UNCパスで正しく動作することを確認してください。".to_owned(),
                ],
            };
            localized.verification_steps = vec![
                "プロジェクトまたはキャッシュの配置を変更した後、同じプロファイルでExecLocusを再診断してください。".to_owned(),
            ];
        }
        "FS002" => {
            "share-firstのプロジェクトがWSLネイティブ領域にあります"
                .clone_into(&mut localized.title);
            "Windows側との共有を優先するプロファイルですが、プロジェクトはWSLネイティブ領域にあります。"
                .clone_into(&mut localized.summary);
            localized.suggested_actions = vec![
                "Linux互換性とファイルシステム特性を優先する場合は、この配置を維持してください。".to_owned(),
                r"Windowsから\\wsl.localhostのUNCパスでプロジェクトへアクセスしてください。".to_owned(),
                "必要なWindowsアプリがUNCパスを安定して利用できない場合だけ移動を検討してください。".to_owned(),
            ];
            localized.verification_steps = vec![
                "プロジェクトを移動した後、share-firstプロファイルでExecLocusを再診断してください。".to_owned(),
            ];
        }
        "PATH001" => {
            localized.title = format!("PATHが別OS層の{role}を選択しています");
            let selected = selected_for_role(report, role).unwrap_or("選択候補");
            localized.summary =
                format!("ネイティブ候補もありますが、PATHは{selected}を選択しています。");
            localized.suggested_actions = vec![
                "設定を変更する前に、現在のシェルのPATH順を確認してください。".to_owned(),
                "再現可能な自動処理では実行ファイルのパスを明示してください。".to_owned(),
            ];
            localized.verification_steps = vec![format!(
                "同じシェルで`{}`を実行してから、ExecLocusを再診断してください。",
                verification_for_role(report, role)
            )];
        }
        "GIT001" => {
            "Gitとプロジェクトが異なるOS層にあります".clone_into(&mut localized.title);
            localized.summary = format!(
                "Gitは{}由来ですが、プロジェクトは{}として分類されています。",
                report
                    .executables
                    .iter()
                    .find(|item| item.role == "git")
                    .and_then(|item| item.selected.as_ref())
                    .map_or("不明", |item| origin_label(item.origin)),
                path_label(report.project.class)
            );
            localized.suggested_actions = vec![
                "プロジェクトの作業環境と同じOS層のGitを優先してください。".to_owned(),
                "切替前に改行、ファイルモード、フック、認証情報ヘルパーを確認してください。"
                    .to_owned(),
            ];
            localized.verification_steps = vec![format!(
                "同じシェルで`{}`を実行してから、ExecLocusを再診断してください。",
                verification_for_role(report, "git")
            )];
        }
        "TOOL001" => {
            "npmは選択されていますがNodeが見つかりません".clone_into(&mut localized.title);
            "このシェルではnpmを解決できますが、Nodeは解決できません。npmラッパーが起動しても、nodeをPATHから直接利用するコマンドやJavaScriptツールは失敗する可能性があります。"
                .clone_into(&mut localized.summary);
            localized.suggested_actions = vec![
                "同じシェルでnpmとNodeの確認コマンドを実行してください。".to_owned(),
                "npmと想定するJavaScript処理の両方が成功する場合だけ、そのラッパーを維持してください。".to_owned(),
                "それ以外の場合は、無関係なPATHを変更する前に、そのシェル用のNodeを初期化または導入してください。".to_owned(),
            ];
            localized.verification_steps = vec![format!(
                "`{}`と`{}`を実行してから、ExecLocusを再診断してください。",
                verification_for_role(report, "npm"),
                verification_for_role(report, "node")
            )];
        }
        _ => {}
    }
    localized
}

fn role_from_finding(finding: &Finding) -> Option<&str> {
    finding.evidence_ids.iter().find_map(|id| {
        id.strip_prefix("executable.")
            .and_then(|value| value.split('.').next())
    })
}

fn product_from_finding(finding: &Finding) -> &'static str {
    if finding
        .evidence_ids
        .iter()
        .any(|id| id.contains("claude_code"))
    {
        "Claude Code"
    } else {
        "Codex"
    }
}

fn verification_for_role<'a>(report: &'a Report, role: &str) -> &'a str {
    report
        .executables
        .iter()
        .find(|item| item.role == role)
        .map_or("確認コマンドを実行", |item| {
            item.verification_command.as_str()
        })
}

fn selected_for_role<'a>(report: &'a Report, role: &str) -> Option<&'a str> {
    report
        .executables
        .iter()
        .find(|item| item.role == role)
        .and_then(|item| item.selected.as_ref())
        .map(|item| item.path.as_str())
}

const fn runtime_label(runtime: RuntimeKind) -> &'static str {
    match runtime {
        RuntimeKind::WindowsNative => "Windowsネイティブ",
        RuntimeKind::Wsl => "WSL",
        RuntimeKind::LinuxNative => "Linuxネイティブ",
        RuntimeKind::Unknown => "不明",
    }
}

const fn origin_label(origin: ExecutableOrigin) -> &'static str {
    match origin {
        ExecutableOrigin::Windows => "Windows",
        ExecutableOrigin::Linux => "Linux",
        ExecutableOrigin::Script => "スクリプト",
        ExecutableOrigin::Unknown => "不明",
    }
}

const fn path_label(class: PathClass) -> &'static str {
    match class {
        PathClass::WindowsNative => "Windowsネイティブ",
        PathClass::WindowsMounted => "Windowsマウント",
        PathClass::WslNative => "WSLネイティブ",
        PathClass::WslUnc => "WSL UNC",
        PathClass::LinuxNative => "Linuxネイティブ",
        PathClass::Unknown => "不明",
    }
}

#[must_use]
pub fn rule_title(definition: &RuleDefinition, language: Language) -> &str {
    if language == Language::English {
        return definition.title;
    }
    match definition.id {
        "ENV001" => "表示中のターミナルとエージェントの実行層が異なる",
        "ENV002" => "WSL実行でWindows実行ファイルが選択される",
        "ENV003" => "エージェントがWindowsとWSLの両方に導入されている",
        "ENV004" => "エージェントの設定または状態がOS層をまたぐ",
        "FS001" => "プロジェクトまたは大きな生成物がWindowsマウント上にある",
        "FS002" => "WSLネイティブのプロジェクトがWindows優先の作業で不便になる可能性がある",
        "PATH001" => "PATH優先順位が別OS層の実行ファイルを選択する",
        "GIT001" => "Gitとプロジェクトが異なるOS層にある",
        "TOOL001" => "npmは選択されるがNodeが見つからない",
        _ => definition.title,
    }
}

#[must_use]
pub fn rule_rationale(definition: &RuleDefinition, language: Language) -> &str {
    if language == Language::English {
        return definition.rationale;
    }
    match definition.id {
        "ENV001" => {
            "表示中のターミナルとエージェントでは、パス、ツール、設定、権限の解決結果が異なる場合があります。"
        }
        "ENV002" => {
            "パス表記、権限、子プロセス、設定場所、パッケージ導入先がLinuxネイティブの想定と異なる場合があります。"
        }
        "ENV003" => {
            "ターミナル、PATH、ランチャーによって選択される導入先が変わり、設定やバージョンが分岐する場合があります。"
        }
        "ENV004" => {
            "書き込み可能な状態がOS層をまたぐと、ロック、権限、改行、性能、同時アクセスが不整合になる場合があります。"
        }
        "FS001" => {
            "Windowsマウントは相互運用に有効ですが、I/O、権限、シンボリックリンク、監視、大小文字の挙動が変わる場合があります。"
        }
        "FS002" => {
            "WindowsからWSL UNCパスを利用できますが、一部のアプリ、ダイアログ、監視、連携では不便になる場合があります。"
        }
        "PATH001" => {
            "選択された版は、ネイティブ候補と異なる設定、パッケージ、パス規則、子プロセス動作を使う場合があります。"
        }
        "GIT001" => {
            "認証、ファイルモード、大小文字、フック、改行、パス処理がOS層によって異なる場合があります。"
        }
        "TOOL001" => {
            "npmはラッパー固有で動作しても、Nodeを直接呼ぶJavaScriptツールにはPATH上のnodeが必要です。"
        }
        _ => definition.rationale,
    }
}

#[must_use]
pub fn rule_required_evidence(definition: &RuleDefinition, language: Language) -> Vec<&str> {
    if language == Language::English {
        return definition.required_evidence.to_vec();
    }
    match definition.id {
        "ENV001" => vec!["ターミナル層", "エージェント実行層", "両者の関係を示す根拠"],
        "ENV002" => vec!["実行層", "解決された実行ファイルパス", "実行形式または由来"],
        "ENV003" => vec!["WindowsとWSL双方の確実なエージェント導入パス"],
        "ENV004" => vec![
            "エージェント実行層",
            "正規化された設定または状態パス",
            "パス分類",
        ],
        "FS001" => vec!["WSL実行", "パス分類", "選択プロファイル"],
        "FS002" => vec!["WSLネイティブのプロジェクトパス", "share-firstプロファイル"],
        "PATH001" => vec!["順序付き候補", "選択実行ファイル", "実行層", "候補の由来"],
        "GIT001" => vec!["解決されたGitの由来", "プロジェクトパス分類"],
        "TOOL001" => vec![
            "選択されたnpm",
            "Nodeが見つからないことを示す完全なシェル根拠",
        ],
        _ => definition.required_evidence.to_vec(),
    }
}

#[must_use]
pub fn rule_suggested_actions(definition: &RuleDefinition, language: Language) -> Vec<&str> {
    if language == Language::English {
        return definition.suggested_actions.to_vec();
    }
    match definition.id {
        "ENV001" => vec![
            "変更前にGit、Node、シェル、プロジェクトパスを比較する",
            "構造化レポートで関係根拠を確認する",
        ],
        "ENV002" => vec![
            "相互運用を意図するなら構成を維持する",
            "Linux動作を意図するならLinuxネイティブ版を優先する",
            "PATH変更前にPATH001を確認する",
        ],
        "ENV003" => vec![
            "変更前にバージョンと解決パスを比較する",
            "意図した二重構成なら維持する",
            "実際の作業方法を確認してから優先度を変更する",
        ],
        "ENV004" => vec![
            "書き込み可能な状態を実行側と同じ層に置く",
            "移植可能と明記された設定だけ共有する",
            "移動前にバックアップする",
        ],
        "FS001" => vec![
            "Windows連携を優先するならマウントを維持する",
            "性能目的の移動前に実測する",
            "選択プロファイルの提案を確認する",
        ],
        "FS002" => vec![
            "Linux特性を優先するならWSLネイティブ配置を維持する",
            "WindowsからWSL UNCパスで参照する",
            "必要なアプリが使えない場合だけ移動する",
        ],
        "PATH001" => vec![
            "変更前に現在のシェルのPATH順を確認する",
            "バージョンと動作を確認する",
            "自動処理ではパスを明示する",
        ],
        "GIT001" => vec![
            "プロジェクトの作業層と同じGitを優先する",
            "切替前に改行、モード、フック、認証を確認する",
        ],
        "TOOL001" => vec![
            "同じシェルでnpmとNodeを確認する",
            "両方が動作する場合だけラッパーを維持する",
            "無関係なPATH変更前にNodeを初期化または導入する",
        ],
        _ => definition.suggested_actions.to_vec(),
    }
}
