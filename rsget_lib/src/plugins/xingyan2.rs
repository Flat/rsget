use crate::Streamable;
use regex::Regex;
use serde_json;

use stream_lib::stream::Stream;
use stream_lib::stream::StreamType;

use crate::utils::downloaders::DownloadClient;

use crate::utils::error::StreamError;
use crate::utils::error::RsgetError;

use chrono::prelude::*;

use std::fs::File;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2RoomInfo {
    rid: String,
    xid: usize,
    name: String,
    xtype: String,
    level: String,
    photo: String,
    picture: String,
    playstatus: String,
    status: String,
    lock_reason: Option<String>,
    personnum: String,
    starttime: String,
    endtime: String,
    label: Vec<String>,
    shareimg: String,
    detail: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2Ads {
    title: String,
    img: String,
    linkurl: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2StreamTrans {
    mid: String,
    small: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2ZL {
    streamurl: String,
    streamtrans: Xingyan2StreamTrans,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2VideoInfo {
    streamurl: String,
    streamtrans: Xingyan2StreamTrans,
    hlsurl: String,
    zl: Vec<Xingyan2ZL>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2HostInfo {
    rid: String,
    nickName: String,
    avatar: String,
    gender: String,
    signature: String,
    is_anchor: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct Xingyan2Info {
    roominfo: Xingyan2RoomInfo,
    videoinfo: Xingyan2VideoInfo,
    hostinfo: Xingyan2HostInfo,
}

#[derive(Clone, Debug)]
pub struct Xingyan2 {
    pub url: String,
    pub room_id: String,
    host_info: Xingyan2Info,
    client: DownloadClient,
}


impl Streamable for Xingyan2 {
    fn new(url: String) -> Result<Box<Xingyan2>, StreamError> {
        let dc = DownloadClient::new()?;
        let room_id_re = Regex::new(r"/([0-9]+)")?;
        let cap = room_id_re.captures(&url)
            .ok_or_else(|| StreamError::Rsget(RsgetError::new("[Xingyan2] Could not find roomid")))?;
        let site_url = format!("https://xingyan.panda.tv/{}", &cap[1]);
        let site_req = dc.make_request(&site_url, None)?;
        let res: Result<String, StreamError> = dc.download_to_string(site_req);
        match res {
            Ok(some) => {
                info!("Unwrapped xinhua");
                let hostinfo_re = Regex::new(r"<script>window.HOSTINFO=(.*);</script>")?;
                let hi_cap = hostinfo_re.captures(&some)
                    .ok_or_else(|| StreamError::Rsget(RsgetError::new("[Xingyan2] Could not find hostinfo")))?;
                let hi: Xingyan2Info = serde_json::from_str(&hi_cap[1])?;
                let tmp = Xingyan2 {
                    url: url.clone(),
                    room_id: String::from(&cap[1]),
                    host_info: hi,
                    client: dc,
                };
                debug!("Xingyan2: \n{:#?}", &tmp);
                Ok(Box::new(tmp))
            },
            Err(why) => {
                Err(why)
            },
        }
    }

    fn get_title(&self) -> Option<String> {
        Some(self.host_info.roominfo.name.clone())
    }

    fn get_author(&self) -> Option<String> {
        Some(self.host_info.hostinfo.nickName.clone())
    }

    fn is_online(&self) -> bool {
        true
        //self.host_info.roominfo.playstatus != "0"
    }

    fn get_stream(&self) -> Result<StreamType, StreamError> {
        Ok(StreamType::Chuncked(self.client.rclient.get(
            &self.host_info.videoinfo.streamurl
        ).build()?))
    }

    fn get_ext(&self) -> String {
        String::from("flv")
    }

    fn get_default_name(&self) -> String {
        let local: DateTime<Local> = Local::now();
        format!(
            "{}-{:04}-{:02}-{:02}-{:02}-{:02}-{}-{}.{}",
            self.room_id,
            local.year(),
            local.month(),
            local.day(),
            local.hour(),
            local.minute(),
            self.get_author().unwrap(),
            self.get_title().unwrap(),
            self.get_ext()
        )
    }

    fn download(&self, path: String) -> Result<u64, StreamError> {
        if !self.is_online() {
            Err(StreamError::Rsget(RsgetError::new("Stream offline")))
        } else {
            println!(
                "{} by {} ({})",
                self.get_title().unwrap(),
                self.get_author().unwrap(),
                self.room_id
            );
            let file = File::create(path)?;
            let stream = Stream::new(self.get_stream()?);
            Ok(stream.write_file(&self.client.rclient, file)?)
        }
    }
}
