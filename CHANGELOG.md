# Unreleased

# 0.17.0
* (breaking) removed `Toast::font(font: FontId)`, this can now be done by using `egui::widget_text::RichText` and `RichText::font`. [#34]
* Added support for `egui::widget_text::WidgetText` in Toasts, this allows for more customization of the text in the toast. [#34]

[#34]: https://github.com/ItsEthra/egui-notify/pull/34

# 0.16.0

* (breaking) Updated to egui `0.29`.
* (breaking) Renamed functions, removed `set_` prefix to fit egui style. [#29]
* Accept either `None` or `Some(duration)` in `set_duration`. [#31]
* Enable shadow beneath toasts using `with_shadow` [#33]

[#29]: https://github.com/ItsEthra/egui-notify/pull/29
[#31]: https://github.com/ItsEthra/egui-notify/pull/31
[#33]: https://github.com/ItsEthra/egui-notify/pull/33
