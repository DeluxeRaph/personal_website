#[derive(Clone, Copy)]
pub(crate) struct BlogPost {
    pub(crate) slug: &'static str,
    pub(crate) title: &'static str,
    pub(crate) date: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) body: &'static str,
}

pub(crate) const POSTS: &[BlogPost] = &[];

pub(crate) fn find_blog_post(slug: &str) -> Option<&'static BlogPost> {
    POSTS.iter().find(|post| post.slug == slug)
}
