use crate::view::pages::manga::ChapterOrder;

use super::{ChapterResponse, Languages, SearchMangaResponse};

#[derive(Clone)]
pub struct MangadexClient {
    api_url_base: String,
    cover_img_url_base: String,
    client: reqwest::Client,
}

impl MangadexClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            api_url_base: "https://api.mangadex.dev".to_string(),
            cover_img_url_base: "https://uploads.mangadex.dev/covers".to_string(),
        }
    }

    pub async fn search_mangas(
        &self,
        search_term: &str,
        page: i32,
    ) -> Result<SearchMangaResponse, reqwest::Error> {
        let offset = (page - 1) * 10;
        let url = format!(
            "{}/manga?title='{}'&includes[]=cover_art&limit=10&offset={}&order[relevance]=desc",
            self.api_url_base,
            search_term.trim(),
            offset,
        );

        self.client.get(url).send().await?.json().await
    }

    pub async fn get_cover_for_manga(
        &self,
        id_manga: &str,
        file_name: &str,
    ) -> Result<bytes::Bytes, reqwest::Error> {
        self.client
            .get(format!(
                "{}/{}/{}",
                self.cover_img_url_base, id_manga, file_name
            ))
            .send()
            .await?
            .bytes()
            .await
    }

    // Todo! implement order by and filter by language and pagination
    pub async fn get_manga_chapters(
        &self,
        id: String,
        page: i32,
        language: Languages,
        order: ChapterOrder,
    ) -> Result<ChapterResponse, reqwest::Error> {
        let language: &str = language.into();
        let page = (page - 1) * 50;

        let order = format!("order[volume]={order}&order[chapter]={order}");
        let endpoint = format!(
            "{}/manga/{}/feed?limit=50&{}&translatedLanguage[]={}&offset=0",
            self.api_url_base, id, order, language
        );

        let reponse = self.client.get(endpoint).send().await?.text().await?;
        Ok(serde_json::from_str(&reponse).unwrap_or_else(|e| panic!("{e}")))
    }
}
