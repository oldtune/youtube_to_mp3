use crate::err::{ApplicationError, Result};
use crate::helpers::marshalling;
use crate::models::manifest::Manifest;

pub struct YoutubeExplorer {}

impl YoutubeExplorer {
    pub async fn get_manifest(video_id: &str) -> Result<Manifest> {
        let player_uri = "https://www.youtube.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8&prettyPrint=true";
        let client = reqwest::Client::new();
        let payload = Self::get_payload(video_id);
        let response = client.post(player_uri).body(payload).send().await?;

        if !response.status().is_success() {
            let bytes = response.bytes().await?.to_vec();
            let error = String::from_utf8(bytes)?;
            return Err(ApplicationError::FailedToGetManifest(error));
        }

        let bytes = &response.bytes().await? as &[u8];
        let string = String::from_utf8_lossy(bytes);
        let manifest: Result<Manifest> = marshalling::deserialize(&string);
        manifest
    }

    pub fn get_payload(video_id: &str) -> String {
        let mut payload = "".to_string();
        payload.push_str(
            r#"{
  "videoId": ""#,
        );
        payload.push_str(video_id);
        payload.push_str(r#"",
  "context": {
    "client": {
      "hl": "en",
      "gl": "VN",
      "remoteHost": "171.235.92.124",
      "deviceMake": "",
      "deviceModel": "",
      "visitorData": "CgtLY2pQb2RkcEdTVSiUmICXBg%3D%3D",
      "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) Gecko/20100101 Firefox/102.0,gzip(gfe)",
      "clientName": "WEB_EMBEDDED_PLAYER",
      "clientVersion": "1.20220724.00.00",
      "osName": "Windows",
      "osVersion": "10.0",
      "originalUrl": "https://www.youtube.com/embed?controls=0&enablejsapi=1&iv_load_policy=3&modestbranding=1&mute=1&rel=0&showinfo=0&origin=https%3A%2F%2Fwww.youtube.com&widgetid=1",
      "screenPixelDensity": 1,
      "platform": "DESKTOP",
      "clientFormFactor": "UNKNOWN_FORM_FACTOR",
      "configInfo": {
        "appInstallData": "CJSYgJcGEMT2_RIQy-z9EhC3y60FENCtrgUQ1IOuBRDl-P0SEMuirgUQpvP9EhC4i64FENi-rQU%3D"
      },
      "screenDensityFloat": 1.0909090909090908,
      "timeZone": "Asia/Ho_Chi_Minh",
      "browserName": "Firefox",
      "browserVersion": "102.0",
      "screenWidthPoints": 0,
      "screenHeightPoints": 0,
      "utcOffsetMinutes": 420,
      "userInterfaceTheme": "USER_INTERFACE_THEME_LIGHT",
      "playerType": "UNIPLAYER",
      "tvAppInfo": { "livingRoomAppMode": "LIVING_ROOM_APP_MODE_UNSPECIFIED" },
      "clientScreen": "ADUNIT"
    },
    "user": { "lockedSafetyMode": false },
    "request": {
      "useSsl": true,
      "internalExperimentFlags": [],
      "consistencyTokenJars": []
    },
    "clientScreenNonce": "MC43ODg5NDY4MTY4MDc1NzQ1",
    "adSignalsInfo": {
      "params": [
        { "key": "dt", "value": "1658850326484" },
        { "key": "flash", "value": "0" },
        { "key": "frm", "value": "1" },
        { "key": "u_tz", "value": "420" },
        { "key": "u_his", "value": "2" },
        { "key": "u_h", "value": "990" },
        { "key": "u_w", "value": "1760" },
        { "key": "u_ah", "value": "953" },
        { "key": "u_aw", "value": "1760" },
        { "key": "u_cd", "value": "24" },
        { "key": "bc", "value": "31" },
        { "key": "bih", "value": "518" },
        { "key": "biw", "value": "1744" },
        { "key": "brdim", "value": "-8,-8,-8,-8,1760,0,1936,1056,0,0" },
        { "key": "vis", "value": "1" },
        { "key": "wgl", "value": "true" },
        { "key": "ca_type", "value": "image" }
      ]
    },
    "thirdParty": { "embedUrl": "https://www.youtube.com/" }
  },
  "playbackContext": {
    "contentPlaybackContext": {
      "html5Preference": "HTML5_PREF_WANTS",
      "lactMilliseconds": "433",
      "referer": "https://www.youtube.com/embed/?controls=0&enablejsapi=1&iv_load_policy=3&modestbranding=1&mute=1&rel=0&showinfo=0&origin=https%3A%2F%2Fwww.youtube.com&widgetid=1",
      "signatureTimestamp": 19198,
      "autoCaptionsDefaultOn": false,
      "mdxContext": {},
      "ancestorOrigins": []
    }
  },
  "cpn": "QZW_YuqFJFzPSxJd",
  "params": "CkoIERAIQhdEZ3pnWXFuMUc3X1F6N3NQaUxDODZBa4oDJyADKAIwBjgIShMIqbTe__KW-QIVP-hzAR0IGA-dUgQIAlgBaAFwPJgDAWAB",
  "captionParams": {}
}

  "#);
        payload
    }
}
