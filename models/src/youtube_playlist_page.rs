#![allow(non_snake_case, non_camel_case_types, clippy::large_enum_variant)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct A11ySkipNavigationButton {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityData1 {
    #[serde(rename = "accessibilityData")]
    pub accessibilitydata: AccessibilityData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionButton {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions {
    #[serde(rename = "addToPlaylistCommand")]
    pub addtoplaylistcommand: AddToPlaylistCommand,
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions1 {
    #[serde(rename = "addToPlaylistCommand")]
    pub addtoplaylistcommand: AddToPlaylistCommand1,
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions2 {
    #[serde(rename = "buttonViewModel")]
    pub buttonviewmodel: ButtonViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions3 {
    #[serde(rename = "toggleButtonViewModel")]
    pub togglebuttonviewmodel: ToggleButtonViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions4 {
    #[serde(rename = "buttonViewModel")]
    pub buttonviewmodel: ButtonViewModel3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions5 {
    #[serde(rename = "buttonViewModel")]
    pub buttonviewmodel: ButtonViewModel4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions6 {
    #[serde(rename = "flexibleActionsViewModel")]
    pub flexibleactionsviewmodel: FlexibleActionsViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions7 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "openPopupAction")]
    pub openpopupaction: OpenPopupAction2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionsEnum {
    Variant1(Actions5),
    Variant2(Actions4),
    Variant3(Actions2),
    Variant4(Actions3),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsRows {
    pub actions: Vec<ActionsEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddToPlaylistCommand {
    #[serde(rename = "listType")]
    pub listtype: String,
    #[serde(rename = "onCreateListCommand")]
    pub oncreatelistcommand: OnCreateListCommand,
    #[serde(rename = "openMiniplayer")]
    pub openminiplayer: bool,
    #[serde(rename = "videoCommand")]
    pub videocommand: VideoCommand,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "videoIds")]
    pub videoids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddToPlaylistCommand1 {
    #[serde(rename = "listType")]
    pub listtype: String,
    #[serde(rename = "onCreateListCommand")]
    pub oncreatelistcommand: OnCreateListCommand,
    #[serde(rename = "openMiniplayer")]
    pub openminiplayer: bool,
    #[serde(rename = "videoCommand")]
    pub videocommand: VideoCommand1,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "videoIds")]
    pub videoids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertWithButtonRenderer {
    #[serde(rename = "dismissButton")]
    pub dismissbutton: DismissButton,
    pub text: Text,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alerts {
    #[serde(rename = "alertWithButtonRenderer")]
    pub alertwithbuttonrenderer: AlertWithButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarStack {
    #[serde(rename = "avatarStackViewModel")]
    pub avatarstackviewmodel: AvatarStackViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarStackViewModel {
    pub avatars: Vec<Avatars>,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext5,
    pub text: Text3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarViewModel {
    #[serde(rename = "avatarImageSize")]
    pub avatarimagesize: String,
    pub image: Image1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Avatars {
    #[serde(rename = "avatarViewModel")]
    pub avatarviewmodel: AvatarViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackButton {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer9,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Background {
    #[serde(rename = "cinematicContainerViewModel")]
    pub cinematiccontainerviewmodel: CinematicContainerViewModel,
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
    #[serde(rename = "browseId")]
    pub browseid: String,
    #[serde(rename = "canonicalBaseUrl")]
    pub canonicalbaseurl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowseEndpoint1 {
    #[serde(rename = "browseId")]
    pub browseid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowseEndpoint2 {
    #[serde(rename = "browseId")]
    pub browseid: String,
    #[serde(rename = "navigationType")]
    pub navigationtype: String,
    pub nofollow: bool,
    pub params: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button1 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button2 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button3 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer7,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer {
    #[serde(rename = "accessibilityData")]
    pub accessibilitydata: AccessibilityData1,
    pub icon: Icon,
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    pub size: String,
    pub style: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer1 {
    pub command: Command,
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    pub size: String,
    pub style: String,
    pub text: Text,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer10 {
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    pub size: String,
    pub style: String,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer11 {
    pub icon: Icon,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint7,
    pub size: String,
    pub style: String,
    #[serde(rename = "targetId")]
    pub targetid: String,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer2 {
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint2,
    pub size: String,
    pub style: String,
    pub text: Text,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer3 {
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint3,
    pub size: String,
    pub style: String,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer4 {
    pub accessibility: AccessibilityData,
    pub icon: Icon,
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: InnertubeCommand2,
    pub size: String,
    pub style: String,
    pub tooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer5 {
    pub accessibility: AccessibilityData,
    pub icon: Icon,
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "serviceEndpoint")]
    pub serviceendpoint: ServiceEndpoint1,
    pub size: String,
    pub style: String,
    pub tooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer6 {
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint5,
    pub size: String,
    pub style: String,
    pub text: Text,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer7 {
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint6,
    pub size: String,
    pub style: String,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer8 {
    pub command: Command2,
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    pub size: String,
    pub style: String,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonRenderer9 {
    pub command: Command2,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel {
    #[serde(rename = "accessibilityText")]
    pub accessibilitytext: String,
    #[serde(rename = "buttonSize")]
    pub buttonsize: String,
    #[serde(rename = "iconName")]
    pub iconname: String,
    #[serde(rename = "isFullWidth")]
    pub isfullwidth: bool,
    #[serde(rename = "onTap")]
    pub ontap: OnTap,
    pub style: String,
    pub title: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel1 {
    #[serde(rename = "accessibilityText")]
    pub accessibilitytext: String,
    #[serde(rename = "buttonSize")]
    pub buttonsize: String,
    #[serde(rename = "iconName")]
    pub iconname: String,
    #[serde(rename = "isFullWidth")]
    pub isfullwidth: bool,
    #[serde(rename = "onTap")]
    pub ontap: OnTap1,
    pub style: String,
    pub tooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel2 {
    #[serde(rename = "accessibilityText")]
    pub accessibilitytext: String,
    #[serde(rename = "buttonSize")]
    pub buttonsize: String,
    #[serde(rename = "iconName")]
    pub iconname: String,
    #[serde(rename = "isFullWidth")]
    pub isfullwidth: bool,
    pub style: String,
    pub tooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel3 {
    #[serde(rename = "accessibilityText")]
    pub accessibilitytext: String,
    #[serde(rename = "buttonSize")]
    pub buttonsize: String,
    #[serde(rename = "iconName")]
    pub iconname: String,
    #[serde(rename = "isFullWidth")]
    pub isfullwidth: bool,
    #[serde(rename = "onTap")]
    pub ontap: OnTap2,
    pub style: String,
    pub tooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonViewModel4 {
    #[serde(rename = "accessibilityText")]
    pub accessibilitytext: String,
    #[serde(rename = "buttonSize")]
    pub buttonsize: String,
    #[serde(rename = "enableIconButton")]
    pub enableiconbutton: bool,
    #[serde(rename = "iconName")]
    pub iconname: String,
    #[serde(rename = "isFullWidth")]
    pub isfullwidth: bool,
    #[serde(rename = "onTap")]
    pub ontap: OnTap5,
    pub state: String,
    pub style: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CinematicContainerViewModel {
    #[serde(rename = "backgroundImageConfig")]
    pub backgroundimageconfig: BackgroundImageConfig,
    pub config: Config,
    #[serde(rename = "gradientColorConfig")]
    pub gradientcolorconfig: Vec<GradientColorConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientResource {
    #[serde(rename = "imageName")]
    pub imagename: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientVeSpec {
    #[serde(rename = "uiType")]
    pub uitype: f64,
    #[serde(rename = "veCounter")]
    pub vecounter: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "signalAction")]
    pub signalaction: SignalAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command1 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "openPopupAction")]
    pub openpopupaction: OpenPopupAction1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command2 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata,
    #[serde(rename = "signalServiceEndpoint")]
    pub signalserviceendpoint: SignalServiceEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext {
    #[serde(rename = "onTap")]
    pub ontap: OnTap3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext1 {
    #[serde(rename = "onTap")]
    pub ontap: OnTap4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext2 {
    #[serde(rename = "onTap")]
    pub ontap: OnTap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandContext3 {
    #[serde(rename = "onTap")]
    pub ontap: OnTap6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandExecutorCommand {
    pub commands: Vec<CommandsEnum>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub webcommandmetadata: WebCommandMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata1 {
    #[serde(rename = "webCommandMetadata")]
    pub webcommandmetadata: WebCommandMetadata1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata2 {
    #[serde(rename = "webCommandMetadata")]
    pub webcommandmetadata: WebCommandMetadata2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata3 {
    #[serde(rename = "webCommandMetadata")]
    pub webcommandmetadata: WebCommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandMetadata4 {
    #[serde(rename = "webCommandMetadata")]
    pub webcommandmetadata: WebCommandMetadata4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRuns {
    pub length: f64,
    #[serde(rename = "onTap")]
    pub ontap: OnTap6,
    #[serde(rename = "startIndex")]
    pub startindex: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "openPopupAction")]
    pub openpopupaction: OpenPopupAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands1 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "playlistVotingRefreshPopupCommand")]
    pub playlistvotingrefreshpopupcommand: PlaylistVotingRefreshPopupCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands2 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata1,
    #[serde(rename = "continuationCommand")]
    pub continuationcommand: ContinuationCommand,
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
    #[serde(rename = "applyClientImageBlur")]
    pub applyclientimageblur: bool,
    #[serde(rename = "colorSourceSizeMultiplier")]
    pub colorsourcesizemultiplier: f64,
    #[serde(rename = "darkThemeBackgroundColor")]
    pub darkthemebackgroundcolor: f64,
    #[serde(rename = "lightThemeBackgroundColor")]
    pub lightthemebackgroundcolor: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config1 {
    #[serde(rename = "webSearchboxConfig")]
    pub websearchboxconfig: WebSearchboxConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "sectionListRenderer")]
    pub sectionlistrenderer: SectionListRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content1 {
    #[serde(rename = "listViewModel")]
    pub listviewmodel: ListViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content2 {
    #[serde(rename = "pageHeaderViewModel")]
    pub pageheaderviewmodel: PageHeaderViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentMetadataViewModel {
    pub delimiter: String,
    #[serde(rename = "metadataRows")]
    pub metadatarows: Vec<MetadataRows>,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentPreviewImageViewModel {
    pub image: Image,
    #[serde(rename = "layoutMode")]
    pub layoutmode: String,
    pub overlays: Vec<Overlays>,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext4,
    pub style: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents {
    #[serde(rename = "playlistVideoRenderer")]
    pub playlistvideorenderer: PlaylistVideoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents1 {
    #[serde(rename = "playlistVideoRenderer")]
    pub playlistvideorenderer: PlaylistVideoRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents2 {
    #[serde(rename = "continuationItemRenderer")]
    pub continuationitemrenderer: ContinuationItemRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents3 {
    #[serde(rename = "playlistVideoListRenderer")]
    pub playlistvideolistrenderer: PlaylistVideoListRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents4 {
    #[serde(rename = "itemSectionRenderer")]
    pub itemsectionrenderer: ItemSectionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents5 {
    #[serde(rename = "continuationItemRenderer")]
    pub continuationitemrenderer: ContinuationItemRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents6 {
    #[serde(rename = "twoColumnBrowseResultsRenderer")]
    pub twocolumnbrowseresultsrenderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentsEnum {
    Variant1(Contents1),
    Variant2(Contents),
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
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandExecutorCommand")]
    pub commandexecutorcommand: CommandExecutorCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuationItemRenderer {
    #[serde(rename = "continuationEndpoint")]
    pub continuationendpoint: ContinuationEndpoint,
    pub trigger: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuationItemRenderer1 {
    #[serde(rename = "continuationEndpoint")]
    pub continuationendpoint: Commands2,
    pub trigger: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlaylistServiceEndpoint {
    pub params: String,
    #[serde(rename = "videoIds")]
    pub videoids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DarkColorPalette {
    #[serde(rename = "iconDisabledColor")]
    pub icondisabledcolor: f64,
    #[serde(rename = "iconInactiveColor")]
    pub iconinactivecolor: f64,
    #[serde(rename = "section2Color")]
    pub section2color: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultButtonViewModel {
    #[serde(rename = "buttonViewModel")]
    pub buttonviewmodel: ButtonViewModel1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "descriptionPreviewViewModel")]
    pub descriptionpreviewviewmodel: DescriptionPreviewViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description1 {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionPreviewViewModel {
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext2,
    #[serde(rename = "truncationText")]
    pub truncationtext: TruncationText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DesktopTopbarRenderer {
    #[serde(rename = "a11ySkipNavigationButton")]
    pub a11yskipnavigationbutton: A11ySkipNavigationButton,
    #[serde(rename = "backButton")]
    pub backbutton: BackButton,
    #[serde(rename = "countryCode")]
    pub countrycode: String,
    #[serde(rename = "forwardButton")]
    pub forwardbutton: BackButton,
    #[serde(rename = "hotkeyDialog")]
    pub hotkeydialog: HotkeyDialog,
    pub logo: Logo,
    pub searchbox: Searchbox,
    #[serde(rename = "topbarButtons")]
    pub topbarbuttons: Vec<TopbarButtonsEnum>,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DismissButton {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DismissButton1 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer10,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicTextViewModel {
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext2,
    pub text: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlexibleActionsViewModel {
    #[serde(rename = "actionsRows")]
    pub actionsrows: Vec<ActionsRows>,
    #[serde(rename = "minimumRowHeight")]
    pub minimumrowheight: f64,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FusionSearchboxRenderer {
    #[serde(rename = "clearButton")]
    pub clearbutton: DismissButton,
    pub config: Config1,
    pub icon: Icon,
    #[serde(rename = "placeholderText")]
    pub placeholdertext: Text1,
    #[serde(rename = "searchEndpoint")]
    pub searchendpoint: SearchEndpoint1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GradientColorConfig {
    #[serde(rename = "darkThemeColor")]
    pub darkthemecolor: f64,
    #[serde(rename = "lightThemeColor")]
    pub lightthemecolor: f64,
    #[serde(rename = "startLocation")]
    pub startlocation: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    #[serde(rename = "pageHeaderRenderer")]
    pub pageheaderrenderer: PageHeaderRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeroImage {
    #[serde(rename = "contentPreviewImageViewModel")]
    pub contentpreviewimageviewmodel: ContentPreviewImageViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialog {
    #[serde(rename = "hotkeyDialogRenderer")]
    pub hotkeydialogrenderer: HotkeyDialogRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogRenderer {
    #[serde(rename = "dismissButton")]
    pub dismissbutton: DismissButton1,
    pub sections: Vec<Sections>,
    pub title: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogSectionOptionRenderer {
    pub hotkey: String,
    pub label: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotkeyDialogSectionOptionRenderer1 {
    pub hotkey: String,
    #[serde(rename = "hotkeyAccessibilityLabel")]
    pub hotkeyaccessibilitylabel: AccessibilityData1,
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
pub enum HotkeyDialogSectionRenderer1_HotkeyDialogSectionRenderer {
    HotkeyDialogSectionRenderer(HotkeyDialogSectionRenderer),
    HotkeyDialogSectionRenderer1(HotkeyDialogSectionRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Html5PlaybackOnesieConfig {
    #[serde(rename = "commonConfig")]
    pub commonconfig: CommonConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Icon {
    #[serde(rename = "iconType")]
    pub icontype: String,
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
    #[serde(rename = "sheetViewModel")]
    pub sheetviewmodel: SheetViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "watchEndpoint")]
    pub watchendpoint: WatchEndpoint3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand1 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata4,
    #[serde(rename = "modalEndpoint")]
    pub modalendpoint: ModalEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand2 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "watchEndpoint")]
    pub watchendpoint: WatchEndpoint4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand3 {
    #[serde(rename = "browseEndpoint")]
    pub browseendpoint: BrowseEndpoint2,
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnertubeCommand4 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "showSheetCommand")]
    pub showsheetcommand: ShowSheetCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemSectionRenderer {
    pub contents: Vec<Contents3>,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    #[serde(rename = "menuServiceItemRenderer")]
    pub menuserviceitemrenderer: MenuServiceItemRenderer_MenuServiceItemRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items1 {
    #[serde(rename = "menuServiceItemRenderer")]
    pub menuserviceitemrenderer: MenuServiceItemRenderer_MenuServiceItemRenderer1_MenuServiceItemRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items2 {
    #[serde(rename = "menuNavigationItemRenderer")]
    pub menunavigationitemrenderer: MenuNavigationItemRenderer_MenuNavigationItemRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items5 {
    #[serde(rename = "playlistSidebarPrimaryInfoRenderer")]
    pub playlistsidebarprimaryinforenderer: PlaylistSidebarPrimaryInfoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items6 {
    #[serde(rename = "playlistSidebarSecondaryInfoRenderer")]
    pub playlistsidebarsecondaryinforenderer: PlaylistSidebarSecondaryInfoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ItemsEnum3 {
    Variant1(Items5),
    Variant2(Items6),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeadingImage {
    pub sources: Vec<Sources>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LengthText {
    pub accessibility: AccessibilityData1,
    #[serde(rename = "simpleText")]
    pub simpletext: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkAlternates {
    #[serde(rename = "hrefUrl")]
    pub hrefurl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItemViewModel {
    #[serde(rename = "leadingImage")]
    pub leadingimage: LeadingImage,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext,
    pub title: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItemViewModel1 {
    #[serde(rename = "leadingImage")]
    pub leadingimage: LeadingImage,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext1,
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
    #[serde(rename = "listItemViewModel")]
    pub listitemviewmodel: ListItemViewModel_ListItemViewModel1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListViewModel {
    #[serde(rename = "listItems")]
    pub listitems: Vec<ListItems>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingContext {
    #[serde(rename = "vssLoggingContext")]
    pub vssloggingcontext: VssLoggingContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingContext1 {
    #[serde(rename = "loggingDirectives")]
    pub loggingdirectives: LoggingDirectives,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingContext2 {
    #[serde(rename = "loggingDirectives")]
    pub loggingdirectives: LoggingDirectives1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingDirectives {
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingDirectives1 {
    #[serde(rename = "clientVeSpec")]
    pub clientvespec: ClientVeSpec,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logo {
    #[serde(rename = "topbarLogoRenderer")]
    pub topbarlogorenderer: TopbarLogoRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainAppWebResponseContext {
    #[serde(rename = "loggedOut")]
    pub loggedout: bool,
    #[serde(rename = "trackingParam")]
    pub trackingparam: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    #[serde(rename = "menuRenderer")]
    pub menurenderer: MenuRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu1 {
    #[serde(rename = "menuRenderer")]
    pub menurenderer: MenuRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu2 {
    #[serde(rename = "menuRenderer")]
    pub menurenderer: MenuRenderer2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuNavigationItemRenderer {
    pub icon: Icon,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: InnertubeCommand3,
    pub text: Text,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuNavigationItemRenderer1 {
    pub icon: Icon,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint4,
    pub text: Text,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MenuNavigationItemRenderer_MenuNavigationItemRenderer1 {
    MenuNavigationItemRenderer(MenuNavigationItemRenderer),
    MenuNavigationItemRenderer1(MenuNavigationItemRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRenderer {
    pub accessibility: AccessibilityData1,
    pub items: Vec<Items1>,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRenderer1 {
    pub accessibility: AccessibilityData1,
    pub items: Vec<Items1>,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRenderer2 {
    pub accessibility: AccessibilityData1,
    pub items: Vec<Items2>,
    #[serde(rename = "targetId")]
    pub targetid: String,
    #[serde(rename = "topLevelButtons")]
    pub toplevelbuttons: Vec<TopLevelButtonsEnum>,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuRequest {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata1,
    #[serde(rename = "signalServiceEndpoint")]
    pub signalserviceendpoint: SignalServiceEndpoint3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuServiceItemRenderer {
    pub icon: Icon,
    #[serde(rename = "serviceEndpoint")]
    pub serviceendpoint: ServiceEndpoint,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuServiceItemRenderer1 {
    #[serde(rename = "hasSeparator")]
    pub hasseparator: bool,
    pub icon: Icon,
    #[serde(rename = "serviceEndpoint")]
    pub serviceendpoint: ServiceEndpoint1,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuServiceItemRenderer2 {
    pub icon: Icon,
    #[serde(rename = "serviceEndpoint")]
    pub serviceendpoint: ServiceEndpoint2,
    pub text: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MenuServiceItemRenderer_MenuServiceItemRenderer1 {
    MenuServiceItemRenderer(MenuServiceItemRenderer),
    MenuServiceItemRenderer1(MenuServiceItemRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MenuServiceItemRenderer_MenuServiceItemRenderer1_MenuServiceItemRenderer2 {
    MenuServiceItemRenderer2(MenuServiceItemRenderer2),
    MenuServiceItemRenderer_MenuServiceItemRenderer1(MenuServiceItemRenderer_MenuServiceItemRenderer1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "contentMetadataViewModel")]
    pub contentmetadataviewmodel: ContentMetadataViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata1 {
    #[serde(rename = "playlistMetadataRenderer")]
    pub playlistmetadatarenderer: PlaylistMetadataRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataParts {
    #[serde(rename = "avatarStack")]
    pub avatarstack: AvatarStack,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataParts1 {
    pub text: Title1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataRows {
    #[serde(rename = "metadataParts")]
    pub metadataparts: VecMetadataParts_VecMetadataParts1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Microformat {
    #[serde(rename = "microformatDataRenderer")]
    pub microformatdatarenderer: MicroformatDataRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MicroformatDataRenderer {
    #[serde(rename = "androidPackage")]
    pub androidpackage: String,
    #[serde(rename = "appName")]
    pub appname: String,
    pub description: String,
    #[serde(rename = "iosAppArguments")]
    pub iosapparguments: String,
    #[serde(rename = "iosAppStoreId")]
    pub iosappstoreid: String,
    #[serde(rename = "linkAlternates")]
    pub linkalternates: Vec<LinkAlternates>,
    pub noindex: bool,
    #[serde(rename = "ogType")]
    pub ogtype: String,
    #[serde(rename = "schemaDotOrgType")]
    pub schemadotorgtype: String,
    #[serde(rename = "siteName")]
    pub sitename: String,
    pub thumbnail: Thumbnail1,
    pub title: String,
    #[serde(rename = "twitterCardType")]
    pub twittercardtype: String,
    #[serde(rename = "twitterSiteHandle")]
    pub twittersitehandle: String,
    pub unlisted: bool,
    #[serde(rename = "urlApplinksAndroid")]
    pub urlapplinksandroid: String,
    #[serde(rename = "urlApplinksIos")]
    pub urlapplinksios: String,
    #[serde(rename = "urlApplinksWeb")]
    pub urlapplinksweb: String,
    #[serde(rename = "urlCanonical")]
    pub urlcanonical: String,
    #[serde(rename = "urlTwitterAndroid")]
    pub urltwitterandroid: String,
    #[serde(rename = "urlTwitterIos")]
    pub urltwitterios: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modal {
    #[serde(rename = "modalWithTitleAndButtonRenderer")]
    pub modalwithtitleandbuttonrenderer: ModalWithTitleAndButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modal1 {
    #[serde(rename = "modalWithTitleAndButtonRenderer")]
    pub modalwithtitleandbuttonrenderer: ModalWithTitleAndButtonRenderer1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modal2 {
    #[serde(rename = "modalWithTitleAndButtonRenderer")]
    pub modalwithtitleandbuttonrenderer: ModalWithTitleAndButtonRenderer2,
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
    #[serde(rename = "showLoadingSpinner")]
    pub showloadingspinner: bool,
    pub style: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "watchEndpoint")]
    pub watchendpoint: WatchEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint1 {
    #[serde(rename = "browseEndpoint")]
    pub browseendpoint: BrowseEndpoint,
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint2 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "signInEndpoint")]
    pub signinendpoint: SignInEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint3 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "signInEndpoint")]
    pub signinendpoint: SignInEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint4 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata4,
    #[serde(rename = "modalEndpoint")]
    pub modalendpoint: ModalEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint5 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "signInEndpoint")]
    pub signinendpoint: SignInEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint6 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata4,
    #[serde(rename = "modalEndpoint")]
    pub modalendpoint: ModalEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationEndpoint7 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "signInEndpoint")]
    pub signinendpoint: SignInEndpoint3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextEndpoint {
    #[serde(rename = "browseEndpoint")]
    pub browseendpoint: BrowseEndpoint1,
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationActionRenderer {
    #[serde(rename = "actionButton")]
    pub actionbutton: ActionButton,
    #[serde(rename = "responseText")]
    pub responsetext: Text,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnCreateListCommand {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata1,
    #[serde(rename = "createPlaylistServiceEndpoint")]
    pub createplaylistserviceendpoint: CreatePlaylistServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: InnertubeCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap1 {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: InnertubeCommand1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap2 {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: ServiceEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap3 {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: InnertubeCommand2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap4 {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: InnertubeCommand3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap5 {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: InnertubeCommand4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnTap6 {
    #[serde(rename = "innertubeCommand")]
    pub innertubecommand: NavigationEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPopupAction {
    #[serde(rename = "beReused")]
    pub bereused: bool,
    pub popup: Popup,
    #[serde(rename = "popupType")]
    pub popuptype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPopupAction1 {
    pub popup: Popup1,
    #[serde(rename = "popupType")]
    pub popuptype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPopupAction2 {
    #[serde(rename = "beReused")]
    pub bereused: bool,
    pub popup: Popup2,
    #[serde(rename = "popupType")]
    pub popuptype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    #[serde(rename = "hotkeyDialogSectionOptionRenderer")]
    pub hotkeydialogsectionoptionrenderer: HotkeyDialogSectionOptionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options1 {
    #[serde(rename = "hotkeyDialogSectionOptionRenderer")]
    pub hotkeydialogsectionoptionrenderer: HotkeyDialogSectionOptionRenderer1_HotkeyDialogSectionOptionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Overlays {
    #[serde(rename = "thumbnailHoverOverlayViewModel")]
    pub thumbnailhoveroverlayviewmodel: ThumbnailHoverOverlayViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageHeaderRenderer {
    pub content: Content2,
    #[serde(rename = "enableSidebarView")]
    pub enablesidebarview: bool,
    #[serde(rename = "pageTitle")]
    pub pagetitle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageHeaderViewModel {
    pub actions: Actions6,
    pub background: Background,
    pub description: Description,
    #[serde(rename = "enableFlexibleActionsButtonsWrapper")]
    pub enableflexibleactionsbuttonswrapper: bool,
    #[serde(rename = "hasTopbarAnimation")]
    pub hastopbaranimation: bool,
    #[serde(rename = "heroImage")]
    pub heroimage: HeroImage,
    pub metadata: Metadata,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext2,
    pub title: Title2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PanelLoadingStrategy {
    #[serde(rename = "inlineContent")]
    pub inlinecontent: InlineContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistMetadataRenderer {
    #[serde(rename = "androidAppindexingLink")]
    pub androidappindexinglink: String,
    #[serde(rename = "iosAppindexingLink")]
    pub iosappindexinglink: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSidebarPrimaryInfoRenderer {
    pub description: Description1,
    pub menu: Menu2,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: InnertubeCommand,
    #[serde(rename = "showMoreText")]
    pub showmoretext: Text1,
    pub stats: Vec<StatsEnum>,
    #[serde(rename = "thumbnailOverlays")]
    pub thumbnailoverlays: Vec<ThumbnailOverlays2>,
    #[serde(rename = "thumbnailRenderer")]
    pub thumbnailrenderer: ThumbnailRenderer,
    pub title: Title3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSidebarRenderer {
    pub items: Vec<ItemsEnum3>,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSidebarSecondaryInfoRenderer {
    pub button: Button3,
    #[serde(rename = "videoOwner")]
    pub videoowner: VideoOwner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoListRenderer {
    #[serde(rename = "canReorder")]
    pub canreorder: bool,
    pub contents: Vec<ContentsEnum>,
    #[serde(rename = "isEditable")]
    pub iseditable: bool,
    #[serde(rename = "playlistId")]
    pub playlistid: String,
    #[serde(rename = "targetId")]
    pub targetid: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoRenderer {
    pub index: Text,
    #[serde(rename = "isPlayable")]
    pub isplayable: bool,
    #[serde(rename = "lengthSeconds")]
    pub lengthseconds: String,
    #[serde(rename = "lengthText")]
    pub lengthtext: LengthText,
    pub menu: Menu,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint,
    #[serde(rename = "shortBylineText")]
    pub shortbylinetext: ShortBylineText,
    pub thumbnail: Thumbnail,
    #[serde(rename = "thumbnailOverlays")]
    pub thumbnailoverlays: Vec<ThumbnailOverlaysEnum>,
    pub title: Title,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "videoInfo")]
    pub videoinfo: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoRenderer1 {
    pub index: Text,
    #[serde(rename = "isPlayable")]
    pub isplayable: bool,
    #[serde(rename = "lengthSeconds")]
    pub lengthseconds: String,
    #[serde(rename = "lengthText")]
    pub lengthtext: LengthText,
    pub menu: Menu1,
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint,
    #[serde(rename = "shortBylineText")]
    pub shortbylinetext: ShortBylineText,
    pub thumbnail: Thumbnail,
    #[serde(rename = "thumbnailOverlays")]
    pub thumbnailoverlays: Vec<ThumbnailOverlaysEnum>,
    pub title: Title,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "videoInfo")]
    pub videoinfo: Text1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVideoThumbnailRenderer {
    pub thumbnail: Thumbnail,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistVotingRefreshPopupCommand {
    pub command: Command1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popup {
    #[serde(rename = "unifiedSharePanelRenderer")]
    pub unifiedsharepanelrenderer: UnifiedSharePanelRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popup1 {
    #[serde(rename = "notificationActionRenderer")]
    pub notificationactionrenderer: NotificationActionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popup2 {
    #[serde(rename = "multiPageMenuRenderer")]
    pub multipagemenurenderer: MultiPageMenuRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Processor {
    #[serde(rename = "borderImageProcessor")]
    pub borderimageprocessor: BorderImageProcessor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext {
    #[serde(rename = "commandContext")]
    pub commandcontext: CommandContext,
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext1 {
    #[serde(rename = "commandContext")]
    pub commandcontext: CommandContext1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext2 {
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext3 {
    #[serde(rename = "commandContext")]
    pub commandcontext: CommandContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext4 {
    #[serde(rename = "accessibilityContext")]
    pub accessibilitycontext: AccessibilityData,
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RendererContext5 {
    #[serde(rename = "accessibilityContext")]
    pub accessibilitycontext: AccessibilityData,
    #[serde(rename = "commandContext")]
    pub commandcontext: CommandContext3,
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContext {
    #[serde(rename = "mainAppWebResponseContext")]
    pub mainappwebresponsecontext: MainAppWebResponseContext,
    #[serde(rename = "serviceTrackingParams")]
    pub servicetrackingparams: Vec<ServiceTrackingParams>,
    #[serde(rename = "webResponseContextExtensionData")]
    pub webresponsecontextextensiondata: WebResponseContextExtensionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runs {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runs1 {
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint1,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runs2 {
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: InnertubeCommand,
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
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "searchEndpoint")]
    pub searchendpoint: SearchEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Searchbox {
    #[serde(rename = "fusionSearchboxRenderer")]
    pub fusionsearchboxrenderer: FusionSearchboxRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SectionListRenderer {
    pub contents: Vec<ContentsEnum1>,
    #[serde(rename = "targetId")]
    pub targetid: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sections {
    #[serde(rename = "hotkeyDialogSectionRenderer")]
    pub hotkeydialogsectionrenderer: HotkeyDialogSectionRenderer1_HotkeyDialogSectionRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata,
    #[serde(rename = "signalServiceEndpoint")]
    pub signalserviceendpoint: SignalServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint1 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata1,
    #[serde(rename = "shareEntityServiceEndpoint")]
    pub shareentityserviceendpoint: ShareEntityServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceEndpoint2 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata,
    #[serde(rename = "signalServiceEndpoint")]
    pub signalserviceendpoint: SignalServiceEndpoint1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceTrackingParams {
    pub params: Vec<Params>,
    pub service: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareEntityServiceEndpoint {
    pub commands: Vec<Commands>,
    #[serde(rename = "serializedShareEntity")]
    pub serializedshareentity: String,
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
    #[serde(rename = "panelLoadingStrategy")]
    pub panelloadingstrategy: PanelLoadingStrategy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sidebar {
    #[serde(rename = "playlistSidebarRenderer")]
    pub playlistsidebarrenderer: PlaylistSidebarRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint {
    #[serde(rename = "idamTag")]
    pub idamtag: String,
    #[serde(rename = "nextEndpoint")]
    pub nextendpoint: NextEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint1 {
    #[serde(rename = "nextEndpoint")]
    pub nextendpoint: NextEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint2 {
    #[serde(rename = "continueAction")]
    pub continueaction: String,
    #[serde(rename = "idamTag")]
    pub idamtag: String,
    #[serde(rename = "nextEndpoint")]
    pub nextendpoint: NextEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInEndpoint3 {
    #[serde(rename = "idamTag")]
    pub idamtag: String,
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
    #[serde(rename = "sizeType")]
    pub sizetype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources {
    #[serde(rename = "clientResource")]
    pub clientresource: ClientResource,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StatsEnum {
    Variant1(Text1),
    Variant2(Text),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    #[serde(rename = "styleType")]
    pub styletype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleRuns {
    pub length: f64,
    #[serde(rename = "startIndex")]
    pub startindex: f64,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleRuns1 {
    pub length: f64,
    #[serde(rename = "startIndex")]
    pub startindex: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleRuns2 {
    #[serde(rename = "fontColor")]
    pub fontcolor: f64,
    pub length: f64,
    #[serde(rename = "startIndex")]
    pub startindex: f64,
    #[serde(rename = "weightLabel")]
    pub weightlabel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabRenderer {
    pub content: Content,
    pub selected: bool,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabs {
    #[serde(rename = "tabRenderer")]
    pub tabrenderer: TabRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    #[serde(rename = "simpleText")]
    pub simpletext: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text1 {
    pub runs: Vec<Runs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text2 {
    pub content: String,
    #[serde(rename = "styleRuns")]
    pub styleruns: Vec<StyleRuns1>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text3 {
    #[serde(rename = "commandRuns")]
    pub commandruns: Vec<CommandRuns>,
    pub content: String,
    #[serde(rename = "styleRuns")]
    pub styleruns: Vec<StyleRuns2>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub thumbnails: Vec<Thumbnails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail1 {
    #[serde(rename = "darkColorPalette")]
    pub darkcolorpalette: DarkColorPalette,
    #[serde(rename = "sampledThumbnailColor")]
    pub sampledthumbnailcolor: SampledThumbnailColor,
    pub thumbnails: Vec<Thumbnails>,
    #[serde(rename = "vibrantColorPalette")]
    pub vibrantcolorpalette: VibrantColorPalette,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailHoverOverlayViewModel {
    pub icon: LeadingImage,
    #[serde(rename = "rendererContext")]
    pub renderercontext: RendererContext3,
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
    #[serde(rename = "thumbnailOverlayTimeStatusRenderer")]
    pub thumbnailoverlaytimestatusrenderer: ThumbnailOverlayTimeStatusRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlays1 {
    #[serde(rename = "thumbnailOverlayNowPlayingRenderer")]
    pub thumbnailoverlaynowplayingrenderer: ThumbnailOverlayNowPlayingRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailOverlays2 {
    #[serde(rename = "thumbnailOverlaySidePanelRenderer")]
    pub thumbnailoverlaysidepanelrenderer: ThumbnailOverlaySidePanelRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ThumbnailOverlaysEnum {
    Variant1(ThumbnailOverlays),
    Variant2(ThumbnailOverlays1),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbnailRenderer {
    #[serde(rename = "playlistVideoThumbnailRenderer")]
    pub playlistvideothumbnailrenderer: PlaylistVideoThumbnailRenderer,
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
    #[serde(rename = "dynamicTextViewModel")]
    pub dynamictextviewmodel: DynamicTextViewModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title3 {
    pub runs: Vec<Runs2>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToggleButtonRenderer {
    #[serde(rename = "accessibilityData")]
    pub accessibilitydata: AccessibilityData1,
    #[serde(rename = "defaultIcon")]
    pub defaulticon: Icon,
    #[serde(rename = "defaultNavigationEndpoint")]
    pub defaultnavigationendpoint: InnertubeCommand1,
    #[serde(rename = "defaultTooltip")]
    pub defaulttooltip: String,
    #[serde(rename = "isDisabled")]
    pub isdisabled: bool,
    #[serde(rename = "isToggled")]
    pub istoggled: bool,
    pub size: Size,
    pub style: Style,
    #[serde(rename = "toggledAccessibilityData")]
    pub toggledaccessibilitydata: AccessibilityData1,
    #[serde(rename = "toggledIcon")]
    pub toggledicon: Icon,
    #[serde(rename = "toggledTooltip")]
    pub toggledtooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToggleButtonViewModel {
    #[serde(rename = "defaultButtonViewModel")]
    pub defaultbuttonviewmodel: DefaultButtonViewModel,
    pub identifier: String,
    #[serde(rename = "isToggled")]
    pub istoggled: bool,
    #[serde(rename = "toggledButtonViewModel")]
    pub toggledbuttonviewmodel: ToggledButtonViewModel,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToggledButtonViewModel {
    #[serde(rename = "buttonViewModel")]
    pub buttonviewmodel: ButtonViewModel2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelButtons {
    #[serde(rename = "toggleButtonRenderer")]
    pub togglebuttonrenderer: ToggleButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelButtons1 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelButtons2 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TopLevelButtonsEnum {
    Variant1(TopLevelButtons1),
    Variant2(TopLevelButtons),
    Variant3(TopLevelButtons2),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topbar {
    #[serde(rename = "desktopTopbarRenderer")]
    pub desktoptopbarrenderer: DesktopTopbarRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarButtons {
    #[serde(rename = "topbarMenuButtonRenderer")]
    pub topbarmenubuttonrenderer: TopbarMenuButtonRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarButtons1 {
    #[serde(rename = "buttonRenderer")]
    pub buttonrenderer: ButtonRenderer11,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TopbarButtonsEnum {
    Variant1(TopbarButtons1),
    Variant2(TopbarButtons),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarLogoRenderer {
    pub endpoint: NextEndpoint,
    #[serde(rename = "iconImage")]
    pub iconimage: Icon,
    #[serde(rename = "overrideEntityKey")]
    pub overrideentitykey: String,
    #[serde(rename = "tooltipText")]
    pub tooltiptext: Text1,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopbarMenuButtonRenderer {
    pub accessibility: AccessibilityData1,
    pub icon: Icon,
    #[serde(rename = "menuRequest")]
    pub menurequest: MenuRequest,
    pub style: String,
    pub tooltip: String,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TruncationText {
    pub content: String,
    #[serde(rename = "styleRuns")]
    pub styleruns: Vec<StyleRuns>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: Vec<Tabs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiedSharePanelRenderer {
    #[serde(rename = "showLoadingSpinner")]
    pub showloadingspinner: bool,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum VecMetadataParts_VecMetadataParts1 {
    VecMetadataParts(Vec<MetadataParts>),
    VecMetadataParts1(Vec<MetadataParts1>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VibrantColorPalette {
    #[serde(rename = "iconInactiveColor")]
    pub iconinactivecolor: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoCommand {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "watchEndpoint")]
    pub watchendpoint: WatchEndpoint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoCommand1 {
    #[serde(rename = "clickTrackingParams")]
    pub clicktrackingparams: String,
    #[serde(rename = "commandMetadata")]
    pub commandmetadata: CommandMetadata2,
    #[serde(rename = "watchEndpoint")]
    pub watchendpoint: WatchEndpoint2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoOwner {
    #[serde(rename = "videoOwnerRenderer")]
    pub videoownerrenderer: VideoOwnerRenderer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoOwnerRenderer {
    #[serde(rename = "navigationEndpoint")]
    pub navigationendpoint: NavigationEndpoint1,
    pub thumbnail: Thumbnail,
    pub title: ShortBylineText,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Visibility {
    pub types: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VssLoggingContext {
    #[serde(rename = "serializedContextData")]
    pub serializedcontextdata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint {
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watchendpointsupportedonesieconfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint1 {
    pub index: f64,
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext,
    pub params: String,
    #[serde(rename = "playerParams")]
    pub playerparams: String,
    #[serde(rename = "playlistId")]
    pub playlistid: String,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watchendpointsupportedonesieconfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint2 {
    #[serde(rename = "playerParams")]
    pub playerparams: String,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watchendpointsupportedonesieconfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint3 {
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext,
    #[serde(rename = "playerParams")]
    pub playerparams: String,
    #[serde(rename = "playlistId")]
    pub playlistid: String,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watchendpointsupportedonesieconfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpoint4 {
    #[serde(rename = "loggingContext")]
    pub loggingcontext: LoggingContext,
    pub params: String,
    #[serde(rename = "playerParams")]
    pub playerparams: String,
    #[serde(rename = "playlistId")]
    pub playlistid: String,
    #[serde(rename = "videoId")]
    pub videoid: String,
    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watchendpointsupportedonesieconfig: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEndpointSupportedOnesieConfig {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playbackonesieconfig: Html5PlaybackOnesieConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata {
    #[serde(rename = "sendPost")]
    pub sendpost: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata1 {
    #[serde(rename = "apiUrl")]
    pub apiurl: String,
    #[serde(rename = "sendPost")]
    pub sendpost: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata2 {
    #[serde(rename = "rootVe")]
    pub rootve: f64,
    pub url: String,
    #[serde(rename = "webPageType")]
    pub webpagetype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata3 {
    #[serde(rename = "apiUrl")]
    pub apiurl: String,
    #[serde(rename = "rootVe")]
    pub rootve: f64,
    pub url: String,
    #[serde(rename = "webPageType")]
    pub webpagetype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebCommandMetadata4 {
    #[serde(rename = "ignoreNavigation")]
    pub ignorenavigation: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponseContextExtensionData {
    #[serde(rename = "hasDecorated")]
    pub hasdecorated: bool,
    #[serde(rename = "ytConfigData")]
    pub ytconfigdata: YtConfigData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSearchboxConfig {
    #[serde(rename = "focusSearchbox")]
    pub focussearchbox: bool,
    #[serde(rename = "hasOnscreenKeyboard")]
    pub hasonscreenkeyboard: bool,
    #[serde(rename = "requestDomain")]
    pub requestdomain: String,
    #[serde(rename = "requestLanguage")]
    pub requestlanguage: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubePlaylistPage {
    pub alerts: Vec<Alerts>,
    pub contents: Contents6,
    pub header: Header,
    pub metadata: Metadata1,
    pub microformat: Microformat,
    #[serde(rename = "responseContext")]
    pub responsecontext: ResponseContext,
    pub sidebar: Sidebar,
    pub topbar: Topbar,
    #[serde(rename = "trackingParams")]
    pub trackingparams: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YtConfigData {
    #[serde(rename = "rootVisualElementType")]
    pub rootvisualelementtype: f64,
    #[serde(rename = "visitorData")]
    pub visitordata: String,
}

