# Report

Between v0.7 and v0.8 of boxy, many new stable features were added. After a few iterations of work the session began to corrupt and by the time the project was tagged with 0.8 it had suffered irrecovrable corruption and many features were broken. @User tried to recover working parts from previous verisons but could only find pieces. In looking at the code he discovered many inefficiencies and problems and began systematically reviewing the code.

This repo is the latest post v0.8 branch with @Users repairs applied mostly to `main.rs`, other files like `theme_engine.rs` have not been analyzed for feature recovery but as of yet there are no compile errors, only warning.

## Partially Recovered Features

1. body_align     - support left(default)|center|right alignment for inner boxy content
2. body_pad_emoji - support alignment of the body content to align with inner title emoji
3. pad_body_above - add N newlines above the body
4. pad_body_below - add N newlines below the body
5. icon_str
6. title_text
7. header_color
8. footer_color


get_themes()
convert_boxy_theme_to_legacy()
get_fallback_legacy_themes()
