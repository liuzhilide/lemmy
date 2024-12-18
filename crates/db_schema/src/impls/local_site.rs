use crate::{
  schema::local_site,
  source::local_site::{LocalSite, LocalSiteInsertForm, LocalSiteUpdateForm},
  utils::{get_conn, DbPool},
};
use diesel::{dsl::insert_into, result::Error};
use diesel_async::RunQueryDsl;
use lemmy_utils::{build_cache, error::LemmyResult, CacheLock};
use std::sync::LazyLock;

impl LocalSite {
  pub async fn create(pool: &mut DbPool<'_>, form: &LocalSiteInsertForm) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    insert_into(local_site::table)
      .values(form)
      .get_result::<Self>(conn)
      .await
  }
  pub async fn read(pool: &mut DbPool<'_>) -> LemmyResult<Self> {
    static CACHE: CacheLock<LocalSite> = LazyLock::new(build_cache);
    Ok(
      CACHE
        .try_get_with((), async {
          let conn = &mut get_conn(pool).await?;
          local_site::table.first(conn).await
        })
        .await?,
    )
  }
  pub async fn update(pool: &mut DbPool<'_>, form: &LocalSiteUpdateForm) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(local_site::table)
      .set(form)
      .get_result::<Self>(conn)
      .await
  }
  pub async fn delete(pool: &mut DbPool<'_>) -> Result<usize, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(local_site::table).execute(conn).await
  }
}
