mod serde_metadata_structs;

use std::process::Command;

use log::{info, /*debug,*/ debug/*, warn*/};
use serde_metadata_structs::{TikTokMeta, PaginationJSONResponse, SignatureJSONResponse};

// const VIDEO_URL_TEMPLATE: &'static str  = "https://m.tiktok.com/share/item/list?secUid={}&id={}&type=1&count=30&minCursor=0&maxCursor={}&shareUid=&lang=";

pub struct User {
  username: String,
}

impl User {
  pub fn new(mut username: String) -> Self {
    debug!("Creating new User");

    if !username.starts_with("@") {
      info!("No @ in username {} adding @", username);
      username = format!("@{}", username);
    }

    Self { username }
  }

  pub fn get_all_video_page_links(self: &Self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let user_page_url = format!("https://tiktok.com/{}", self.username);

    debug!("GETing: {}", user_page_url);
    let user_page_body = reqwest::blocking::get(&user_page_url)?.text()?;

    debug!("Extracting: {}", user_page_url);
    let metadata_raw = extract_meta_from_page(user_page_body);
    let metadata: TikTokMeta = serde_json::from_str(&metadata_raw)?;


    let mut video_page_links: Vec<String> = Vec::new();
    // paginate the page and get all the video page links
    let mut paging: bool = true;
    let mut max_cursor = String::from("0");

    while paging {
      let mut pagination_url : String = format!("https://m.tiktok.com/api/item_list/?count=30&id={}1&type=5&secUid={}&maxCursor={}&minCursor=0&sourceType=12&appId=1233", 
        metadata.props.page_props.user_data.user_id,
        metadata.props.page_props.user_data.sec_uid, 
        max_cursor
      );

      self.sign_url(&mut pagination_url);

      debug!("GETing: {}", pagination_url);
      let pagination_response_raw = reqwest::blocking::get(&pagination_url)?.text()?;
      debug!("Deserializing: {}", pagination_url);
      let pagination_response: PaginationJSONResponse = serde_json::from_str(&pagination_response_raw)?;

      for item in pagination_response.items {
        video_page_links.push(format!("https://www.tiktok.com/{}/video/{}", self.username, item.id));
      }
      
      if max_cursor == pagination_response.max_cursor {
        paging = false;
      } else {
        max_cursor = pagination_response.max_cursor;
      }
    }
    
    let mut video_page_media_links: Vec<String> = Vec::new();
    // extract all the media links from the page
    
    // for page in video_page_links {
    //   let detail_url = format!("https://m.tiktok.com/api/item/detail/?page_referer={}&did={}&itemId={}",
    //                   user_page_url);
      
    //   // playAddr

    // }
    
    video_page_media_links.append(&mut video_page_links);
    Ok(video_page_media_links)
  }

  // fn extract_media_links_from_video_page(page: String) -> Vec<String> {
  //   Vec::new()
  // }
  fn sign_url(&self, url: &mut String) {
    debug!("Signing: {}", url);

    let output = if cfg!(target_os = "windows") {
      Command::new("cmd")
              .args(&["/C", &format!("node ./external/tiktok-signature/browser.js \"{}\"", url)])
              // .args(&["/C", "node "])
              .output()
              .expect("failed to execute process")
    } else {
      Command::new("sh")
              .arg("-c")
              .arg(&format!("node ./external/tiktok-signature/browser.js \"{}\"", url))
              .output()
              .expect("failed to execute process")
    };
  
    let sig_json_raw = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);

      debug!("{}",err);
    let sig_json: SignatureJSONResponse = serde_json::from_str(&sig_json_raw).expect("Signature Failed!");

    *url = format!("{}{}{}{}{}", url, "&verifyFp=", &sig_json.verify_fp, "&_signature=", &sig_json.signature);
  }
}

//TODO-Unwraps
fn extract_meta_from_page(mut html: String) -> String {
  debug!("Finding data tag");
  let script_start_i = html
    .find("<script id=\"__NEXT_DATA__\" type=\"application/json\" crossorigin=\"anonymous\">")
    .expect("Find meta script tag failed");
    
  debug!("Splitting off start");
  let mut script = html.split_off(script_start_i);

  debug!("Finding end of tag block");
  let script_end_i = script.find("</script>").expect("Find '</script>' failed");

  debug!("Removing end of html");
  script.truncate(script_end_i);

  debug!("Finding end of open tag");
  let script_tag_end_i = script.find(">").expect("Find '>' failed");

  debug!("Splitting off opening tag, returning metadata json");
  script.split_off(script_tag_end_i + 1)
}
