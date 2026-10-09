//! Type-scale tokens.
//!
//! Tightened in v0.5 from the splash 12-step scale (9-26pt) to a 7-step
//! modern scale (11-26pt). Each token has a fixed role; the C3 render
//! crate uses these to size labels, chip text, body copy and titles.

pub const F_CAPTION: f32 = 11.0; // chip / 脚注 (was 9-10)
pub const F_META: f32    = 12.0; // 元信息 (was 11)
pub const F_BODY: f32    = 14.0; // 正文 (was 12-13)
pub const F_BODY_LG: f32 = 16.0; // 强调正文 (was 14-15)
pub const F_SECTION: f32 = 18.0; // 区段标题 (was 16)
pub const F_TITLE: f32   = 22.0; // 屏幕大标题 (was 18-20)
pub const F_HERO: f32    = 26.0; // hero / app 标题 (was 22-26)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_is_monotonic() {
        assert!(F_CAPTION < F_META);
        assert!(F_META    < F_BODY);
        assert!(F_BODY    < F_BODY_LG);
        assert!(F_BODY_LG < F_SECTION);
        assert!(F_SECTION < F_TITLE);
        assert!(F_TITLE   < F_HERO);
    }
}
