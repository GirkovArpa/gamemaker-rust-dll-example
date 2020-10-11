var foo = external_define("gamemaker_rust_dll_example.dll", "foo", dll_cdecl, ty_real, 1, ty_real);
external_call(foo, 123);

var bar = external_define("gamemaker_rust_dll_example.dll", "bar", dll_cdecl, ty_real, 1, ty_string);
external_call(bar, "baz");

var print_window_handle = external_define("gamemaker_rust_dll_example.dll", "print_window_handle", dll_cdecl, ty_real, 1, ty_string);
external_call(print_window_handle, string(window_handle()));

var foo_bar_baz = external_define("gamemaker_rust_dll_example.dll", "foo_bar_baz", dll_cdecl, ty_string, 0);
show_debug_message(external_call(foo_bar_baz));

var elite_number = external_define("gamemaker_rust_dll_example.dll", "elite_number", dll_cdecl, ty_real, 0);
show_debug_message(external_call(elite_number));