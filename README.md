## Web

```
# one time
cargo install --locked trunk

# debug (http://127.0.0.1:8080)
trunk serve 

# release (/dist folder)
trunk build --release
```


## Notes

I tried to use twiggy to analyze the size, but didn't get anything so far.


(Note: use the git version via ``cargo install twiggy --git https://github.com/rustwasm/twiggy``)

Running twiggy on the result of ``trunk build --release`` gives:

```
> twiggy top -n 20  .\dist\min-sized-egui-bin-7fbf6c62dc5d322f_bg.wasm        
 Shallow Bytes │ Shallow % │ Item
───────────────┼───────────┼─────────────────────
         77071 ┊    13.09% ┊ code[1]
         27993 ┊     4.76% ┊ code[0]
         22843 ┊     3.88% ┊ data[30]
         22621 ┊     3.84% ┊ code[6]
         13698 ┊     2.33% ┊ code[2]
          9427 ┊     1.60% ┊ code[3]
          6941 ┊     1.18% ┊ data[6]
          5928 ┊     1.01% ┊ code[4]
          5419 ┊     0.92% ┊ code[10]
          5280 ┊     0.90% ┊ code[7]
          5173 ┊     0.88% ┊ code[5]
          4674 ┊     0.79% ┊ code[8]
          4322 ┊     0.73% ┊ data[43]
          4138 ┊     0.70% ┊ code[9]
          3973 ┊     0.67% ┊ code[18]
          3871 ┊     0.66% ┊ data[57]
          3742 ┊     0.64% ┊ code[13]
          3712 ┊     0.63% ┊ code[12]
          3499 ┊     0.59% ┊ data[52]
          3489 ┊     0.59% ┊ code[15]
        350833 ┊    59.60% ┊ ... and 2131 more.
        588647 ┊   100.00% ┊ Σ [2151 Total Rows]
```

Disabling ``strip`` in cargo.toml and then running twiggy on the result of ``cargo build --release --target wasm32-unknown-unknown`` gives:

```
> twiggy top  .\target\wasm32-unknown-unknown\release\min-sized-egui-bin.wasm -n 20             
 Shallow Bytes │ Shallow % │ Item
───────────────┼───────────┼──────────────────────────────────────────────────────────────────────────────
        426957 ┊    24.36% ┊ custom section '__wasm_bindgen_unstable'
        334943 ┊    19.11% ┊ "function names" subsection
         59698 ┊     3.41% ┊ data segment ".rodata"
         32750 ┊     1.87% ┊ egui::context::Context::write::h64c1362511a5b5e7
         32409 ┊     1.85% ┊ min_sized_egui_bin::main::{{closure}}::hb84ff90f490ae539
         20670 ┊     1.18% ┊ ttf_parser::Face::parse_tables::h5fb98c6e5b731a28
         18521 ┊     1.06% ┊ image::codecs::png::PngEncoder<W>::encode_inner::h7847993200c2a4f2
         17393 ┊     0.99% ┊ eframe::web::app_runner::AppRunner::logic::h467c9064109ee5a7
         14137 ┊     0.81% ┊ epaint::text::fonts::FontsAndCache::layout_job::hfc38cfe5767b7727
         12896 ┊     0.74% ┊ egui::context::Context::write::hd99c12a72348fb8a
          6622 ┊     0.38% ┊ <min_sized_egui_bin::app::App as eframe::epi::App>::update::h8311961f617f7ada
          6254 ┊     0.36% ┊ epaint::tessellator::Tessellator::tessellate_shape::h9995b790e71e8098
          5284 ┊     0.30% ┊ <flate2::mem::Compress as flate2::zio::Ops>::run_vec::h0c1362954c2544d8
          4792 ┊     0.27% ┊ eframe::web::web_painter_glow::init_webgl1::hf75c308f4ccbe51c
          4642 ┊     0.26% ┊ egui::context::Context::create_widget::h0259097eebbb959d
          4588 ┊     0.26% ┊ ttf_parser::Face::parse::hfb0995efff04fa47
          4513 ┊     0.26% ┊ epaint::tessellator::stroke_and_fill_path::heaff7e6f1722d393
          4377 ┊     0.25% ┊ egui::data::key::Key::from_name::hcc7d943fdd4a614a
          4289 ┊     0.24% ┊ eframe::web::web_painter_glow::init_webgl2::hd69204a8bea415ba
          4275 ┊     0.24% ┊ eframe::web::app_runner::AppRunner::paint::h29fafbe818887414
        733011 ┊    41.81% ┊ ... and 7594 more.
       1753021 ┊   100.00% ┊ Σ [7614 Total Rows]
```