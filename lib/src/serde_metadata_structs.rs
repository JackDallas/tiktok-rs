use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TikTokMeta {
    pub props: Props,
    pub page: String,
    pub query: Query,
    pub build_id: String,
    pub asset_prefix: String,
    pub is_fallback: bool,
    pub custom_server: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Props {
    pub initial_props: InitialProps,
    pub page_props: PageProps,
    pub pathname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialProps {
    pub status_code: i64,
    #[serde(rename = "$wid")]
    pub wid: String,
    #[serde(rename = "$host")]
    pub host: String,
    #[serde(rename = "$pageUrl")]
    pub page_url: String,
    #[serde(rename = "$isMobile")]
    pub is_mobile: bool,
    #[serde(rename = "$isAndroid")]
    pub is_android: bool,
    #[serde(rename = "$isIOS")]
    pub is_ios: bool,
    #[serde(rename = "$brand")]
    pub brand: String,
    #[serde(rename = "$client")]
    pub client: String,
    #[serde(rename = "$cookieConsent")]
    pub cookie_consent: ::serde_json::Value,
    #[serde(rename = "$region")]
    pub region: String,
    #[serde(rename = "$language")]
    pub language: String,
    #[serde(rename = "$originalLanguage")]
    pub original_language: String,
    #[serde(rename = "$languageList")]
    pub language_list: Vec<LanguageList>,
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$reflowType")]
    pub reflow_type: String,
    #[serde(rename = "$appId")]
    pub app_id: i64,
    #[serde(rename = "$botType")]
    pub bot_type: String,
    #[serde(rename = "$appType")]
    pub app_type: String,
    #[serde(rename = "$sgOpen")]
    pub sg_open: bool,
    #[serde(rename = "$downloadLink")]
    pub download_link: DownloadLink,
    #[serde(rename = "$legalList")]
    pub legal_list: Vec<LegalList>,
    #[serde(rename = "$gray")]
    pub gray: Gray,
    #[serde(rename = "$config")]
    pub config: Config,
    #[serde(rename = "$baseURL")]
    pub base_url: String,
    #[serde(rename = "$abTestVersion")]
    pub ab_test_version: AbTestVersion,
    #[serde(rename = "$isInCookiePolicyArea")]
    pub is_in_cookie_policy_area: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageList {
    pub value: String,
    pub alias: String,
    pub label: String,
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub value: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLink {
    pub amazon: Amazon,
    pub google: Google,
    pub apple: Apple,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amazon {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Google {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apple {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalList {
    pub title: String,
    pub key: String,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gray {
    pub upload: bool,
    pub tea: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub covid_banner: CovidBanner,
    pub bytedance_link: BytedanceLink,
    pub kr_popup: KrPopup,
    pub sg_open: SgOpen,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovidBanner {
    pub open: bool,
    pub url: String,
    pub background: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BytedanceLink {
    pub link_visible: bool,
    pub override_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KrPopup {
    pub open: bool,
    pub url: String,
    pub text: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgOpen {
    #[serde(rename = "SG_OPEN")]
    pub sg_open: bool,
    #[serde(rename = "BOT_THRESHOLD")]
    pub bot_threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbTestVersion {
    pub version_name: String,
    pub parameters: Parameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(rename = "download_guide")]
    pub download_guide: DownloadGuide,
    #[serde(rename = "autoplay_test")]
    pub autoplay_test: AutoplayTest,
    #[serde(rename = "side_nav_test")]
    pub side_nav_test: SideNavTest,
    #[serde(rename = "open_url_test")]
    pub open_url_test: OpenUrlTest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadGuide {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoplayTest {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideNavTest {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrlTest {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProps {
    pub server_code: i64,
    pub unique_id: String,
    pub page_state: PageState,
    pub share_user: ShareUser,
    pub share_meta: ShareMeta,
    pub lang_list: Vec<LangList>,
    pub test_id: String,
    pub meta_params: MetaParams,
    pub item_list: Vec<::serde_json::Value>,
    pub desc_video: DescVideo,
    pub status_code: i64,
    pub user_info: UserInfo,
    pub user_data: UserData,
    pub feed_config: FeedConfig,
    #[serde(rename = "isSSR")]
    pub is_ssr: bool,
    pub page_options: PageOptions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageState {
    pub region_app_id: i64,
    pub os: String,
    pub region: String,
    #[serde(rename = "baseURL")]
    pub base_url: String,
    pub app_type: String,
    pub full_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareUser {
    pub sec_uid: String,
    pub user_id: String,
    pub unique_id: String,
    pub nick_name: String,
    pub signature: String,
    pub covers: Vec<::serde_json::Value>,
    pub covers_medium: Vec<::serde_json::Value>,
    pub covers_larger: Vec<::serde_json::Value>,
    pub is_secret: bool,
    pub relation: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareMeta {
    pub title: String,
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LangList {
    pub value: String,
    pub alias: String,
    pub label: String,
    pub children: Vec<Children2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    pub value: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaParams {
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub robots_content: String,
    pub canonical_href: String,
    pub alternate_href: ::serde_json::Value,
    pub amphtml_href: ::serde_json::Value,
    pub applicable_device: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescVideo {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user: User,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub unique_id: String,
    pub nickname: String,
    pub avatar_thumb: String,
    pub avatar_medium: String,
    pub signature: String,
    pub verified: bool,
    pub secret: bool,
    pub sec_uid: String,
    pub open_favorite: bool,
    pub relation: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub following_count: i64,
    pub follower_count: i64,
    pub heart_count: String,
    pub video_count: i64,
    pub digg_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub sec_uid: String,
    pub user_id: String,
    pub is_secret: bool,
    pub unique_id: String,
    pub nick_name: String,
    pub signature: String,
    pub covers: Vec<String>,
    pub covers_medium: Vec<String>,
    pub following: i64,
    pub fans: i64,
    pub heart: String,
    pub video: i64,
    pub verified: bool,
    pub digg: i64,
    pub ftc: bool,
    pub relation: i64,
    pub open_favorite: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConfig {
    pub page_type: i64,
    pub sec_uid: String,
    pub id: String,
    pub show_avatar: bool,
    pub empty_tip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageOptions {
    pub footer: Footer,
    pub header: Header,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Footer {
    pub hidden: bool,
    pub show_download: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub show_upload: bool,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub unique_id: String,
    #[serde(rename = "$initialProps")]
    pub initial_props: InitialProps2,
    #[serde(rename = "aa59d67c2123f094d0d6798ffe651c4d")]
    pub aa59_d67_c2123_f094_d0_d6798_ffe651_c4_d: Aa59D67C2123F094D0D6798Ffe651C4D,
    #[serde(rename = "abtest_metrics_BF884930EC2D14D3")]
    pub abtest_metrics_bf884930_ec2_d14_d3: AbtestMetricsBf884930Ec2D14D3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialProps2 {
    #[serde(rename = "$wid")]
    pub wid: String,
    #[serde(rename = "$host")]
    pub host: String,
    #[serde(rename = "$pageUrl")]
    pub page_url: String,
    #[serde(rename = "$isMobile")]
    pub is_mobile: bool,
    #[serde(rename = "$isAndroid")]
    pub is_android: bool,
    #[serde(rename = "$isIOS")]
    pub is_ios: bool,
    #[serde(rename = "$brand")]
    pub brand: String,
    #[serde(rename = "$client")]
    pub client: String,
    #[serde(rename = "$cookieConsent")]
    pub cookie_consent: ::serde_json::Value,
    #[serde(rename = "$region")]
    pub region: String,
    #[serde(rename = "$language")]
    pub language: String,
    #[serde(rename = "$originalLanguage")]
    pub original_language: String,
    #[serde(rename = "$languageList")]
    pub language_list: Vec<LanguageList2>,
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$reflowType")]
    pub reflow_type: String,
    #[serde(rename = "$appId")]
    pub app_id: i64,
    #[serde(rename = "$botType")]
    pub bot_type: String,
    #[serde(rename = "$appType")]
    pub app_type: String,
    #[serde(rename = "$sgOpen")]
    pub sg_open: bool,
    #[serde(rename = "$downloadLink")]
    pub download_link: DownloadLink2,
    #[serde(rename = "$legalList")]
    pub legal_list: Vec<LegalList2>,
    #[serde(rename = "$gray")]
    pub gray: Gray2,
    #[serde(rename = "$config")]
    pub config: Config2,
    #[serde(rename = "$baseURL")]
    pub base_url: String,
    #[serde(rename = "$abTestVersion")]
    pub ab_test_version: AbTestVersion2,
    #[serde(rename = "$isInCookiePolicyArea")]
    pub is_in_cookie_policy_area: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageList2 {
    pub value: String,
    pub alias: String,
    pub label: String,
    pub children: Vec<Children3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children3 {
    pub value: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLink2 {
    pub amazon: Amazon2,
    pub google: Google2,
    pub apple: Apple2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amazon2 {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Google2 {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apple2 {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalList2 {
    pub title: String,
    pub key: String,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gray2 {
    pub upload: bool,
    pub tea: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config2 {
    pub covid_banner: CovidBanner2,
    pub bytedance_link: BytedanceLink2,
    pub kr_popup: KrPopup2,
    pub sg_open: SgOpen2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovidBanner2 {
    pub open: bool,
    pub url: String,
    pub background: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BytedanceLink2 {
    pub link_visible: bool,
    pub override_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KrPopup2 {
    pub open: bool,
    pub url: String,
    pub text: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgOpen2 {
    #[serde(rename = "SG_OPEN")]
    pub sg_open: bool,
    #[serde(rename = "BOT_THRESHOLD")]
    pub bot_threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbTestVersion2 {
    pub version_name: String,
    pub parameters: Parameters2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters2 {
    #[serde(rename = "download_guide")]
    pub download_guide: DownloadGuide2,
    #[serde(rename = "autoplay_test")]
    pub autoplay_test: AutoplayTest2,
    #[serde(rename = "side_nav_test")]
    pub side_nav_test: SideNavTest2,
    #[serde(rename = "open_url_test")]
    pub open_url_test: OpenUrlTest2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadGuide2 {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoplayTest2 {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideNavTest2 {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrlTest2 {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aa59D67C2123F094D0D6798Ffe651C4D {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbtestMetricsBf884930Ec2D14D3 {
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationJSONResponse {
    pub status_code: i64,
    pub items: Vec<Item>,
    pub has_more: bool,
    pub max_cursor: String,
    pub min_cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub desc: String,
    pub create_time: i64,
    pub video: Video,
    pub author: Author,
    pub music: Music,
    pub stats: StatsPagiation,
    pub original_item: bool,
    pub offical_item: bool,
    #[serde(default)]
    pub text_extra: Vec<TextExtra>,
    pub secret: bool,
    pub for_friend: bool,
    pub digged: bool,
    pub item_comment_status: i64,
    pub show_not_pass: bool,
    pub vl1: bool,
    pub item_mute: bool,
    #[serde(default)]
    pub challenges: Vec<Challenge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub height: i64,
    pub width: i64,
    pub duration: i64,
    pub ratio: String,
    pub cover: String,
    pub origin_cover: String,
    pub dynamic_cover: String,
    pub play_addr: String,
    pub download_addr: String,
    pub share_cover: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub unique_id: String,
    pub nickname: String,
    pub avatar_thumb: String,
    pub avatar_medium: String,
    pub avatar_larger: String,
    pub signature: String,
    pub verified: bool,
    pub sec_uid: String,
    pub secret: bool,
    pub relation: i64,
    pub open_favorite: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Music {
    pub id: String,
    pub title: String,
    pub play_url: String,
    pub cover_thumb: String,
    pub cover_medium: String,
    pub cover_large: String,
    pub author_name: String,
    pub original: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPagiation {
    pub digg_count: i64,
    pub share_count: i64,
    pub comment_count: i64,
    pub play_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextExtra {
    pub aweme_id: String,
    pub start: i64,
    pub end: i64,
    pub hashtag_name: String,
    pub hashtag_id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub user_id: String,
    pub is_commerce: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub profile_thumb: String,
    pub profile_medium: String,
    pub profile_larger: String,
    pub cover_thumb: String,
    pub cover_medium: String,
    pub cover_larger: String,
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureJSONResponse {
    pub signature: String,
    pub verify_fp: String,
    pub cookies: String,
}
