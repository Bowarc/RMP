#![allow(non_snake_case, non_camel_case_types)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct A11ySkipNavigationButton {
    pub buttonRenderer: ButtonRenderer8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityData1 {
    pub accessibilityData: AccessibilityData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionButton {
    pub buttonRenderer: ButtonRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions {
    pub addToPlaylistCommand: AddToPlaylistCommand,
    pub clickTrackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions1 {
    pub addToPlaylistCommand: AddToPlaylistCommand1,
    pub clickTrackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions2 {
    pub buttonViewModel: ButtonViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions3 {
    pub toggleButtonViewModel: ToggleButtonViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions4 {
    pub buttonViewModel: ButtonViewModel3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions5 {
    pub buttonViewModel: ButtonViewModel4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions6 {
    pub flexibleActionsViewModel: FlexibleActionsViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions7 {
    pub clickTrackingParams: String,
    pub openPopupAction: OpenPopupAction2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionsEnum {
    Variant1(Actions5),
    Variant2(Actions2),
    Variant3(Actions3),
    Variant4(Actions4),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsRows {
    pub actions: Vec<ActionsEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddToPlaylistCommand {
    pub listType: String,
    pub onCreateListCommand: OnCreateListCommand,
    pub openMiniplayer: bool,
    pub videoCommand: VideoCommand,
    pub videoId: String,
    pub videoIds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddToPlaylistCommand1 {
    pub listType: String,
    pub onCreateListCommand: OnCreateListCommand,
    pub openMiniplayer: bool,
    pub videoCommand: VideoCommand1,
    pub videoId: String,
    pub videoIds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertWithButtonRenderer {
    pub dismissButton: DismissButton,
    pub text: Text,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alerts {
    pub alertWithButtonRenderer: AlertWithButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarStack {
    pub avatarStackViewModel: AvatarStackViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarStackViewModel {
    pub avatars: Vec<Avatars>,
    pub rendererContext: RendererContext5,
    pub text: Text3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarViewModel {
    pub avatarImageSize: String,
    pub image: Image1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Avatars {
    pub avatarViewModel: AvatarViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackButton {
    pub buttonRenderer: ButtonRenderer9,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Background {
    pub cinematicContainerViewModel: CinematicContainerViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackgroundImageConfig {
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BorderImageProcessor {
    pub circular: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowseEndpoint {
    pub browseId: String,
    pub canonicalBaseUrl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowseEndpoint1 {
    pub browseId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowseEndpoint2 {
    pub browseId: String,
    pub navigationType: String,
    pub nofollow: bool,
    pub params: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    pub buttonRenderer: ButtonRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button1 {
    pub buttonRenderer: ButtonRenderer3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button2 {
    pub buttonRenderer: ButtonRenderer6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button3 {
    pub buttonRenderer: ButtonRenderer7,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer {
    pub accessibilityData: AccessibilityData1,
    pub icon: Icon,
    pub isDisabled: bool,
    pub size: String,
    pub style: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer1 {
    pub command: Command,
    pub isDisabled: bool,
    pub size: String,
    pub style: String,
    pub text: Text,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer10 {
    pub isDisabled: bool,
    pub size: String,
    pub style: String,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer11 {
    pub icon: Icon,
    pub navigationEndpoint: NavigationEndpoint7,
    pub size: String,
    pub style: String,
    pub targetId: String,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer2 {
    pub isDisabled: bool,
    pub navigationEndpoint: NavigationEndpoint2,
    pub size: String,
    pub style: String,
    pub text: Text,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer3 {
    pub isDisabled: bool,
    pub navigationEndpoint: NavigationEndpoint3,
    pub size: String,
    pub style: String,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer4 {
    pub accessibility: AccessibilityData,
    pub icon: Icon,
    pub isDisabled: bool,
    pub navigationEndpoint: InnertubeCommand2,
    pub size: String,
    pub style: String,
    pub tooltip: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer5 {
    pub accessibility: AccessibilityData,
    pub icon: Icon,
    pub isDisabled: bool,
    pub serviceEndpoint: ServiceEndpoint1,
    pub size: String,
    pub style: String,
    pub tooltip: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer6 {
    pub isDisabled: bool,
    pub navigationEndpoint: NavigationEndpoint5,
    pub size: String,
    pub style: String,
    pub text: Text,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer7 {
    pub isDisabled: bool,
    pub navigationEndpoint: NavigationEndpoint6,
    pub size: String,
    pub style: String,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer8 {
    pub command: Command2,
    pub isDisabled: bool,
    pub size: String,
    pub style: String,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer9 {
    pub command: Command2,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel {
    pub accessibilityText: String,
    pub buttonSize: String,
    pub iconName: String,
    pub isFullWidth: bool,
    pub onTap: OnTap,
    pub style: String,
    pub title: String,
    pub trackingParams: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel1 {
    pub accessibilityText: String,
    pub buttonSize: String,
    pub iconName: String,
    pub isFullWidth: bool,
    pub onTap: OnTap1,
    pub style: String,
    pub tooltip: String,
    pub trackingParams: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel2 {
    pub accessibilityText: String,
    pub buttonSize: String,
    pub iconName: String,
    pub isFullWidth: bool,
    pub style: String,
    pub tooltip: String,
    pub trackingParams: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel3 {
    pub accessibilityText: String,
    pub buttonSize: String,
    pub iconName: String,
    pub isFullWidth: bool,
    pub onTap: OnTap2,
    pub style: String,
    pub tooltip: String,
    pub trackingParams: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel4 {
    pub accessibilityText: String,
    pub buttonSize: String,
    pub enableIconButton: bool,
    pub iconName: String,
    pub isFullWidth: bool,
    pub onTap: OnTap5,
    pub state: String,
    pub style: String,
    pub trackingParams: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CinematicContainerViewModel {
    pub backgroundImageConfig: BackgroundImageConfig,
    pub config: Config,
    pub gradientColorConfig: Vec<GradientColorConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientResource {
    pub imageName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientVeSpec {
    pub uiType: f64,
    pub veCounter: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub clickTrackingParams: String,
    pub signalAction: SignalAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command1 {
    pub clickTrackingParams: String,
    pub openPopupAction: OpenPopupAction1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command2 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata,
    pub signalServiceEndpoint: SignalServiceEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext {
    pub onTap: OnTap3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext1 {
    pub onTap: OnTap4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext2 {
    pub onTap: OnTap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext3 {
    pub onTap: OnTap6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandExecutorCommand {
    pub commands: Vec<CommandsEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata {
    pub webCommandMetadata: WebCommandMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata1 {
    pub webCommandMetadata: WebCommandMetadata1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata2 {
    pub webCommandMetadata: WebCommandMetadata2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata3 {
    pub webCommandMetadata: WebCommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata4 {
    pub webCommandMetadata: WebCommandMetadata4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRuns {
    pub length: f64,
    pub onTap: OnTap6,
    pub startIndex: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands {
    pub clickTrackingParams: String,
    pub openPopupAction: OpenPopupAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands1 {
    pub clickTrackingParams: String,
    pub playlistVotingRefreshPopupCommand: PlaylistVotingRefreshPopupCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands2 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata1,
    pub continuationCommand: ContinuationCommand,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommandsEnum {
    Variant1(Commands1),
    Variant2(Commands2),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub applyClientImageBlur: bool,
    pub colorSourceSizeMultiplier: f64,
    pub darkThemeBackgroundColor: f64,
    pub lightThemeBackgroundColor: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config1 {
    pub webSearchboxConfig: WebSearchboxConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub sectionListRenderer: SectionListRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content1 {
    pub listViewModel: ListViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content2 {
    pub pageHeaderViewModel: PageHeaderViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentMetadataViewModel {
    pub delimiter: String,
    pub metadataRows: Vec<MetadataRows>,
    pub rendererContext: RendererContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentPreviewImageViewModel {
    pub image: Image,
    pub layoutMode: String,
    pub overlays: Vec<Overlays>,
    pub rendererContext: RendererContext4,
    pub style: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents {
    pub playlistVideoRenderer: PlaylistVideoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents1 {
    pub playlistVideoRenderer: PlaylistVideoRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents2 {
    pub continuationItemRenderer: ContinuationItemRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents3 {
    pub playlistVideoListRenderer: PlaylistVideoListRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents4 {
    pub itemSectionRenderer: ItemSectionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents5 {
    pub continuationItemRenderer: ContinuationItemRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents6 {
    pub twoColumnBrowseResultsRenderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentsEnum {
    Variant1(Contents),
    Variant2(Contents1),
    Variant3(Contents2),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentsEnum1 {
    Variant1(Contents5),
    Variant2(Contents4),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuationCommand {
    pub request: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuationEndpoint {
    pub clickTrackingParams: String,
    pub commandExecutorCommand: CommandExecutorCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuationItemRenderer {
    pub continuationEndpoint: ContinuationEndpoint,
    pub trigger: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuationItemRenderer1 {
    pub continuationEndpoint: Commands2,
    pub trigger: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlaylistServiceEndpoint {
    pub params: String,
    pub videoIds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DarkColorPalette {
    pub iconDisabledColor: f64,
    pub iconInactiveColor: f64,
    pub section2Color: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultButtonViewModel {
    pub buttonViewModel: ButtonViewModel1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub descriptionPreviewViewModel: DescriptionPreviewViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description1 {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionPreviewViewModel {
    pub rendererContext: RendererContext2,
    pub truncationText: TruncationText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DesktopTopbarRenderer {
    pub a11ySkipNavigationButton: A11ySkipNavigationButton,
    pub backButton: BackButton,
    pub countryCode: String,
    pub forwardButton: BackButton,
    pub hotkeyDialog: HotkeyDialog,
    pub logo: Logo,
    pub searchbox: Searchbox,
    pub topbarButtons: Vec<TopbarButtonsEnum>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DismissButton {
    pub buttonRenderer: ButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DismissButton1 {
    pub buttonRenderer: ButtonRenderer10,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicTextViewModel {
    pub rendererContext: RendererContext2,
    pub text: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlexibleActionsViewModel {
    pub actionsRows: Vec<ActionsRows>,
    pub minimumRowHeight: f64,
    pub rendererContext: RendererContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FusionSearchboxRenderer {
    pub clearButton: DismissButton,
    pub config: Config1,
    pub icon: Icon,
    pub placeholderText: Text1,
    pub searchEndpoint: SearchEndpoint1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GradientColorConfig {
    pub darkThemeColor: f64,
    pub lightThemeColor: f64,
    pub startLocation: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub pageHeaderRenderer: PageHeaderRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeroImage {
    pub contentPreviewImageViewModel: ContentPreviewImageViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialog {
    pub hotkeyDialogRenderer: HotkeyDialogRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogRenderer {
    pub dismissButton: DismissButton1,
    pub sections: Vec<Sections>,
    pub title: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogSectionOptionRenderer {
    pub hotkey: String,
    pub label: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogSectionOptionRenderer1 {
    pub hotkey: String,
    pub hotkeyAccessibilityLabel: AccessibilityData1,
    pub label: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HotkeyDialogSectionOptionRenderer1_HotkeyDialogSectionOptionRenderer {
    HotkeyDialogSectionOptionRenderer(HotkeyDialogSectionOptionRenderer),
    HotkeyDialogSectionOptionRenderer1(HotkeyDialogSectionOptionRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogSectionRenderer {
    pub options: Vec<Options1>,
    pub title: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogSectionRenderer1 {
    pub options: Vec<Options1>,
    pub title: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HotkeyDialogSectionRenderer_HotkeyDialogSectionRenderer1 {
    HotkeyDialogSectionRenderer(HotkeyDialogSectionRenderer),
    HotkeyDialogSectionRenderer1(HotkeyDialogSectionRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Html5PlaybackOnesieConfig {
    pub commonConfig: CommonConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Icon {
    pub iconType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub sources: Vec<Thumbnails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image1 {
    pub processor: Processor,
    pub sources: Vec<Thumbnails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineContent {
    pub sheetViewModel: SheetViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub watchEndpoint: WatchEndpoint3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand1 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata4,
    pub modalEndpoint: ModalEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand2 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub watchEndpoint: WatchEndpoint4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand3 {
    pub browseEndpoint: BrowseEndpoint2,
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand4 {
    pub clickTrackingParams: String,
    pub showSheetCommand: ShowSheetCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemSectionRenderer {
    pub contents: Vec<Contents3>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    pub menuServiceItemRenderer: MenuServiceItemRenderer1_MenuServiceItemRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items1 {
    pub menuServiceItemRenderer: MenuServiceItemRenderer1_MenuServiceItemRenderer_MenuServiceItemRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items2 {
    pub menuNavigationItemRenderer: MenuNavigationItemRenderer1_MenuNavigationItemRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items5 {
    pub playlistSidebarPrimaryInfoRenderer: PlaylistSidebarPrimaryInfoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items6 {
    pub playlistSidebarSecondaryInfoRenderer: PlaylistSidebarSecondaryInfoRenderer,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ItemsEnum3 {
    Variant1(Items6),
    Variant2(Items5),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeadingImage {
    pub sources: Vec<Sources>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthText {
    pub accessibility: AccessibilityData1,
    pub simpleText: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkAlternates {
    pub hrefUrl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItemViewModel {
    pub leadingImage: LeadingImage,
    pub rendererContext: RendererContext,
    pub title: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItemViewModel1 {
    pub leadingImage: LeadingImage,
    pub rendererContext: RendererContext1,
    pub title: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ListItemViewModel_ListItemViewModel1 {
    ListItemViewModel(ListItemViewModel),
    ListItemViewModel1(ListItemViewModel1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItems {
    pub listItemViewModel: ListItemViewModel_ListItemViewModel1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListViewModel {
    pub listItems: Vec<ListItems>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingContext {
    pub vssLoggingContext: VssLoggingContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingContext1 {
    pub loggingDirectives: LoggingDirectives,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingContext2 {
    pub loggingDirectives: LoggingDirectives1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingDirectives {
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingDirectives1 {
    pub clientVeSpec: ClientVeSpec,
    pub trackingParams: String,
    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logo {
    pub topbarLogoRenderer: TopbarLogoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainAppWebResponseContext {
    pub loggedOut: bool,
    pub trackingParam: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub menuRenderer: MenuRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu1 {
    pub menuRenderer: MenuRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu2 {
    pub menuRenderer: MenuRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuNavigationItemRenderer {
    pub icon: Icon,
    pub navigationEndpoint: InnertubeCommand3,
    pub text: Text,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuNavigationItemRenderer1 {
    pub icon: Icon,
    pub navigationEndpoint: NavigationEndpoint4,
    pub text: Text,
    pub trackingParams: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MenuNavigationItemRenderer1_MenuNavigationItemRenderer {
    MenuNavigationItemRenderer(MenuNavigationItemRenderer),
    MenuNavigationItemRenderer1(MenuNavigationItemRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRenderer {
    pub accessibility: AccessibilityData1,
    pub items: Vec<Items1>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRenderer1 {
    pub accessibility: AccessibilityData1,
    pub items: Vec<Items1>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRenderer2 {
    pub accessibility: AccessibilityData1,
    pub items: Vec<Items2>,
    pub targetId: String,
    pub topLevelButtons: Vec<TopLevelButtonsEnum>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRequest {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata1,
    pub signalServiceEndpoint: SignalServiceEndpoint3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuServiceItemRenderer {
    pub icon: Icon,
    pub serviceEndpoint: ServiceEndpoint,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuServiceItemRenderer1 {
    pub hasSeparator: bool,
    pub icon: Icon,
    pub serviceEndpoint: ServiceEndpoint1,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuServiceItemRenderer2 {
    pub icon: Icon,
    pub serviceEndpoint: ServiceEndpoint2,
    pub text: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MenuServiceItemRenderer1_MenuServiceItemRenderer {
    MenuServiceItemRenderer(MenuServiceItemRenderer),
    MenuServiceItemRenderer1(MenuServiceItemRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MenuServiceItemRenderer1_MenuServiceItemRenderer_MenuServiceItemRenderer2 {
    MenuServiceItemRenderer1_MenuServiceItemRenderer(MenuServiceItemRenderer1_MenuServiceItemRenderer),
    MenuServiceItemRenderer2(MenuServiceItemRenderer2),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub contentMetadataViewModel: ContentMetadataViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata1 {
    pub playlistMetadataRenderer: PlaylistMetadataRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataParts {
    pub avatarStack: AvatarStack,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataParts1 {
    pub text: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataRows {
    pub metadataParts: VecMetadataParts1_VecMetadataParts,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Microformat {
    pub microformatDataRenderer: MicroformatDataRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MicroformatDataRenderer {
    pub androidPackage: String,
    pub appName: String,
    pub description: String,
    pub iosAppArguments: String,
    pub iosAppStoreId: String,
    pub linkAlternates: Vec<LinkAlternates>,
    pub noindex: bool,
    pub ogType: String,
    pub schemaDotOrgType: String,
    pub siteName: String,
    pub thumbnail: Thumbnail1,
    pub title: String,
    pub twitterCardType: String,
    pub twitterSiteHandle: String,
    pub unlisted: bool,
    pub urlApplinksAndroid: String,
    pub urlApplinksIos: String,
    pub urlApplinksWeb: String,
    pub urlCanonical: String,
    pub urlTwitterAndroid: String,
    pub urlTwitterIos: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modal {
    pub modalWithTitleAndButtonRenderer: ModalWithTitleAndButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modal1 {
    pub modalWithTitleAndButtonRenderer: ModalWithTitleAndButtonRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modal2 {
    pub modalWithTitleAndButtonRenderer: ModalWithTitleAndButtonRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModalEndpoint {
    pub modal: Modal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModalEndpoint1 {
    pub modal: Modal1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModalEndpoint2 {
    pub modal: Modal2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModalWithTitleAndButtonRenderer {
    pub button: Button,
    pub content: Text,
    pub title: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModalWithTitleAndButtonRenderer1 {
    pub button: Button1,
    pub content: Text,
    pub title: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModalWithTitleAndButtonRenderer2 {
    pub button: Button2,
    pub content: Text,
    pub title: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiPageMenuRenderer {
    pub showLoadingSpinner: bool,
    pub style: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub watchEndpoint: WatchEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint1 {
    pub browseEndpoint: BrowseEndpoint,
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint2 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub signInEndpoint: SignInEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint3 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub signInEndpoint: SignInEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint4 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata4,
    pub modalEndpoint: ModalEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint5 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub signInEndpoint: SignInEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint6 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata4,
    pub modalEndpoint: ModalEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint7 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub signInEndpoint: SignInEndpoint3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextEndpoint {
    pub browseEndpoint: BrowseEndpoint1,
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationActionRenderer {
    pub actionButton: ActionButton,
    pub responseText: Text,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnCreateListCommand {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata1,
    pub createPlaylistServiceEndpoint: CreatePlaylistServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap {
    pub innertubeCommand: InnertubeCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap1 {
    pub innertubeCommand: InnertubeCommand1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap2 {
    pub innertubeCommand: ServiceEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap3 {
    pub innertubeCommand: InnertubeCommand2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap4 {
    pub innertubeCommand: InnertubeCommand3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap5 {
    pub innertubeCommand: InnertubeCommand4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap6 {
    pub innertubeCommand: NavigationEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPopupAction {
    pub beReused: bool,
    pub popup: Popup,
    pub popupType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPopupAction1 {
    pub popup: Popup1,
    pub popupType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPopupAction2 {
    pub beReused: bool,
    pub popup: Popup2,
    pub popupType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub hotkeyDialogSectionOptionRenderer: HotkeyDialogSectionOptionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options1 {
    pub hotkeyDialogSectionOptionRenderer: HotkeyDialogSectionOptionRenderer1_HotkeyDialogSectionOptionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Overlays {
    pub thumbnailHoverOverlayViewModel: ThumbnailHoverOverlayViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageHeaderRenderer {
    pub content: Content2,
    pub enableSidebarView: bool,
    pub pageTitle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageHeaderViewModel {
    pub actions: Actions6,
    pub background: Background,
    pub description: Description,
    pub enableFlexibleActionsButtonsWrapper: bool,
    pub hasTopbarAnimation: bool,
    pub heroImage: HeroImage,
    pub metadata: Metadata,
    pub rendererContext: RendererContext2,
    pub title: Title2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PanelLoadingStrategy {
    pub inlineContent: InlineContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistMetadataRenderer {
    pub androidAppindexingLink: String,
    pub iosAppindexingLink: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSidebarPrimaryInfoRenderer {
    pub description: Description1,
    pub menu: Menu2,
    pub navigationEndpoint: InnertubeCommand,
    pub showMoreText: Text1,
    pub stats: Vec<StatsEnum>,
    pub thumbnailOverlays: Vec<ThumbnailOverlays2>,
    pub thumbnailRenderer: ThumbnailRenderer,
    pub title: Title3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSidebarRenderer {
    pub items: Vec<ItemsEnum3>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSidebarSecondaryInfoRenderer {
    pub button: Button3,
    pub videoOwner: VideoOwner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoListRenderer {
    pub canReorder: bool,
    pub contents: Vec<ContentsEnum>,
    pub isEditable: bool,
    pub playlistId: String,
    pub targetId: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoRenderer {
    pub index: Text,
    pub isPlayable: bool,
    pub lengthSeconds: String,
    pub lengthText: LengthText,
    pub menu: Menu,
    pub navigationEndpoint: NavigationEndpoint,
    pub shortBylineText: ShortBylineText,
    pub thumbnail: Thumbnail,
    pub thumbnailOverlays: Vec<ThumbnailOverlaysEnum>,
    pub title: Title,
    pub trackingParams: String,
    pub videoId: String,
    pub videoInfo: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoRenderer1 {
    pub index: Text,
    pub isPlayable: bool,
    pub lengthSeconds: String,
    pub lengthText: LengthText,
    pub menu: Menu1,
    pub navigationEndpoint: NavigationEndpoint,
    pub shortBylineText: ShortBylineText,
    pub thumbnail: Thumbnail,
    pub thumbnailOverlays: Vec<ThumbnailOverlaysEnum>,
    pub title: Title,
    pub trackingParams: String,
    pub videoId: String,
    pub videoInfo: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoThumbnailRenderer {
    pub thumbnail: Thumbnail,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVotingRefreshPopupCommand {
    pub command: Command1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popup {
    pub unifiedSharePanelRenderer: UnifiedSharePanelRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popup1 {
    pub notificationActionRenderer: NotificationActionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popup2 {
    pub multiPageMenuRenderer: MultiPageMenuRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Processor {
    pub borderImageProcessor: BorderImageProcessor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext {
    pub commandContext: CommandContext,
    pub loggingContext: LoggingContext1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext1 {
    pub commandContext: CommandContext1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext2 {
    pub loggingContext: LoggingContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext3 {
    pub commandContext: CommandContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext4 {
    pub accessibilityContext: AccessibilityData,
    pub loggingContext: LoggingContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext5 {
    pub accessibilityContext: AccessibilityData,
    pub commandContext: CommandContext3,
    pub loggingContext: LoggingContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContext {
    pub mainAppWebResponseContext: MainAppWebResponseContext,
    pub serviceTrackingParams: Vec<ServiceTrackingParams>,
    pub webResponseContextExtensionData: WebResponseContextExtensionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runs {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runs1 {
    pub navigationEndpoint: NavigationEndpoint1,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runs2 {
    pub navigationEndpoint: InnertubeCommand,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SampledThumbnailColor {
    pub blue: f64,
    pub green: f64,
    pub red: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchEndpoint {
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchEndpoint1 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub searchEndpoint: SearchEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Searchbox {
    pub fusionSearchboxRenderer: FusionSearchboxRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SectionListRenderer {
    pub contents: Vec<ContentsEnum1>,
    pub targetId: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sections {
    pub hotkeyDialogSectionRenderer: HotkeyDialogSectionRenderer_HotkeyDialogSectionRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata,
    pub signalServiceEndpoint: SignalServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint1 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata1,
    pub shareEntityServiceEndpoint: ShareEntityServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint2 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata,
    pub signalServiceEndpoint: SignalServiceEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceTrackingParams {
    pub params: Vec<Params>,
    pub service: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareEntityServiceEndpoint {
    pub commands: Vec<Commands>,
    pub serializedShareEntity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SheetViewModel {
    pub content: Content1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortBylineText {
    pub runs: Vec<Runs1>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShowSheetCommand {
    pub panelLoadingStrategy: PanelLoadingStrategy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sidebar {
    pub playlistSidebarRenderer: PlaylistSidebarRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint {
    pub idamTag: String,
    pub nextEndpoint: NextEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint1 {
    pub nextEndpoint: NextEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint2 {
    pub continueAction: String,
    pub idamTag: String,
    pub nextEndpoint: NextEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint3 {
    pub idamTag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalAction {
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalServiceEndpoint {
    pub actions: Vec<Actions>,
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalServiceEndpoint1 {
    pub actions: Vec<Actions1>,
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalServiceEndpoint2 {
    pub actions: Vec<Command>,
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalServiceEndpoint3 {
    pub actions: Vec<Actions7>,
    pub signal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub sizeType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources {
    pub clientResource: ClientResource,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StatsEnum {
    Variant1(Text1),
    Variant2(Text),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    pub styleType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleRuns {
    pub length: f64,
    pub startIndex: f64,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleRuns1 {
    pub length: f64,
    pub startIndex: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleRuns2 {
    pub fontColor: f64,
    pub length: f64,
    pub startIndex: f64,
    pub weightLabel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabRenderer {
    pub content: Content,
    pub selected: bool,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabs {
    pub tabRenderer: TabRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub simpleText: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text1 {
    pub runs: Vec<Runs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text2 {
    pub content: String,
    pub styleRuns: Vec<StyleRuns1>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text3 {
    pub commandRuns: Vec<CommandRuns>,
    pub content: String,
    pub styleRuns: Vec<StyleRuns2>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub thumbnails: Vec<Thumbnails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail1 {
    pub darkColorPalette: DarkColorPalette,
    pub sampledThumbnailColor: SampledThumbnailColor,
    pub thumbnails: Vec<Thumbnails>,
    pub vibrantColorPalette: VibrantColorPalette,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailHoverOverlayViewModel {
    pub icon: LeadingImage,
    pub rendererContext: RendererContext3,
    pub style: String,
    pub text: Text2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlayNowPlayingRenderer {
    pub text: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlaySidePanelRenderer {
    pub icon: Icon,
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlayTimeStatusRenderer {
    pub style: String,
    pub text: LengthText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlays {
    pub thumbnailOverlayTimeStatusRenderer: ThumbnailOverlayTimeStatusRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlays1 {
    pub thumbnailOverlayNowPlayingRenderer: ThumbnailOverlayNowPlayingRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlays2 {
    pub thumbnailOverlaySidePanelRenderer: ThumbnailOverlaySidePanelRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ThumbnailOverlaysEnum {
    Variant1(ThumbnailOverlays),
    Variant2(ThumbnailOverlays1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailRenderer {
    pub playlistVideoThumbnailRenderer: PlaylistVideoThumbnailRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnails {
    pub height: f64,
    pub url: String,
    pub width: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    pub accessibility: AccessibilityData1,
    pub runs: Vec<Runs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title1 {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title2 {
    pub dynamicTextViewModel: DynamicTextViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title3 {
    pub runs: Vec<Runs2>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToggleButtonRenderer {
    pub accessibilityData: AccessibilityData1,
    pub defaultIcon: Icon,
    pub defaultNavigationEndpoint: InnertubeCommand1,
    pub defaultTooltip: String,
    pub isDisabled: bool,
    pub isToggled: bool,
    pub size: Size,
    pub style: Style,
    pub toggledAccessibilityData: AccessibilityData1,
    pub toggledIcon: Icon,
    pub toggledTooltip: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToggleButtonViewModel {
    pub defaultButtonViewModel: DefaultButtonViewModel,
    pub identifier: String,
    pub isToggled: bool,
    pub toggledButtonViewModel: ToggledButtonViewModel,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToggledButtonViewModel {
    pub buttonViewModel: ButtonViewModel2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelButtons {
    pub toggleButtonRenderer: ToggleButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelButtons1 {
    pub buttonRenderer: ButtonRenderer4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelButtons2 {
    pub buttonRenderer: ButtonRenderer5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TopLevelButtonsEnum {
    Variant1(TopLevelButtons),
    Variant2(TopLevelButtons1),
    Variant3(TopLevelButtons2),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topbar {
    pub desktopTopbarRenderer: DesktopTopbarRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarButtons {
    pub topbarMenuButtonRenderer: TopbarMenuButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarButtons1 {
    pub buttonRenderer: ButtonRenderer11,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TopbarButtonsEnum {
    Variant1(TopbarButtons),
    Variant2(TopbarButtons1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarLogoRenderer {
    pub endpoint: NextEndpoint,
    pub iconImage: Icon,
    pub overrideEntityKey: String,
    pub tooltipText: Text1,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarMenuButtonRenderer {
    pub accessibility: AccessibilityData1,
    pub icon: Icon,
    pub menuRequest: MenuRequest,
    pub style: String,
    pub tooltip: String,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TruncationText {
    pub content: String,
    pub styleRuns: Vec<StyleRuns>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: Vec<Tabs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiedSharePanelRenderer {
    pub showLoadingSpinner: bool,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum VecMetadataParts1_VecMetadataParts {
    VecMetadataParts(Vec<MetadataParts>),
    VecMetadataParts1(Vec<MetadataParts1>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VibrantColorPalette {
    pub iconInactiveColor: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoCommand {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub watchEndpoint: WatchEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoCommand1 {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata2,
    pub watchEndpoint: WatchEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoOwner {
    pub videoOwnerRenderer: VideoOwnerRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoOwnerRenderer {
    pub navigationEndpoint: NavigationEndpoint1,
    pub thumbnail: Thumbnail,
    pub title: ShortBylineText,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Visibility {
    pub types: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VssLoggingContext {
    pub serializedContextData: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint {
    pub videoId: String,
    pub watchEndpointSupportedOnesieConfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint1 {
    pub index: f64,
    pub loggingContext: LoggingContext,
    pub params: String,
    pub playerParams: String,
    pub playlistId: String,
    pub videoId: String,
    pub watchEndpointSupportedOnesieConfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint2 {
    pub playerParams: String,
    pub videoId: String,
    pub watchEndpointSupportedOnesieConfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint3 {
    pub loggingContext: LoggingContext,
    pub playerParams: String,
    pub playlistId: String,
    pub videoId: String,
    pub watchEndpointSupportedOnesieConfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint4 {
    pub loggingContext: LoggingContext,
    pub params: String,
    pub playerParams: String,
    pub playlistId: String,
    pub videoId: String,
    pub watchEndpointSupportedOnesieConfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpointSupportedOnesieConfig {
    pub html5PlaybackOnesieConfig: Html5PlaybackOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata {
    pub sendPost: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata1 {
    pub apiUrl: String,
    pub sendPost: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata2 {
    pub rootVe: f64,
    pub url: String,
    pub webPageType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata3 {
    pub apiUrl: String,
    pub rootVe: f64,
    pub url: String,
    pub webPageType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata4 {
    pub ignoreNavigation: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponseContextExtensionData {
    pub hasDecorated: bool,
    pub ytConfigData: YtConfigData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSearchboxConfig {
    pub focusSearchbox: bool,
    pub hasOnscreenKeyboard: bool,
    pub requestDomain: String,
    pub requestLanguage: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubePlaylistPage {
    pub alerts: Vec<Alerts>,
    pub contents: Contents6,
    pub header: Header,
    pub metadata: Metadata1,
    pub microformat: Microformat,
    pub responseContext: ResponseContext,
    pub sidebar: Sidebar,
    pub topbar: Topbar,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YtConfigData {
    pub rootVisualElementType: f64,
    pub visitorData: String,
}

