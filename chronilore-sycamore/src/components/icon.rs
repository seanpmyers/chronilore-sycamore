pub type SvgIcon = &'static str;

pub const DNA_SVG_HTML: SvgIcon = r#"
	<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
		<rect width="256" height="256" fill="none" />
		<line x1="96" y1="48" x2="192" y2="48" fill="none" stroke="currentColor" stroke-linecap="round"
			stroke-linejoin="round" stroke-width="16" />
		<path d="M64,24V51.5a72,72,0,0,0,39.8,64.4l48.4,24.2A72,72,0,0,1,192,204.5V232" fill="none"
			stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
		<line x1="64" y1="208" x2="160" y2="208" fill="none" stroke="currentColor" stroke-linecap="round"
			stroke-linejoin="round" stroke-width="16" />
		<line x1="107.89" y1="80" x2="186.12" y2="80" fill="none" stroke="currentColor"
			stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
		<line x1="69.88" y1="176" x2="148.11" y2="176" fill="none" stroke="currentColor"
			stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
		<path d="M93.38,146.47A72,72,0,0,0,64,204.5V232" fill="none" stroke="currentColor"
			stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
		<path d="M192,24V51.5a72,72,0,0,1-29.38,58" fill="none" stroke="currentColor"
			stroke-linecap="round" stroke-linejoin="round" stroke-width="16" />
	</svg>
"#;

pub const MOON_SVG_HTML: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
	stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-moon">
	<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
</svg>
"#;

pub const SUN_SVG_HTML: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
	stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-sun">
	<circle cx="12" cy="12" r="5"></circle>
	<line x1="12" y1="1" x2="12" y2="3"></line>
	<line x1="12" y1="21" x2="12" y2="23"></line>
	<line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
	<line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
	<line x1="1" y1="12" x2="3" y2="12"></line>
	<line x1="21" y1="12" x2="23" y2="12"></line>
	<line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
	<line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
</svg>
"#;

pub const CHEVRON_UP_SVG_HTML: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
	stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-chevron-up">
	<polyline points="18 15 12 9 6 15"></polyline>
</svg>
"#;

pub const CHEVRON_DOWN_SVG_HTML: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
	stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-chevron-down">
	<polyline points="6 9 12 15 18 9"></polyline>
</svg>
"#;

pub const CLOSE_X_SVG_HTML: SvgIcon = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
	<rect width="256" height="256" fill="none" />
	<line x1="200" y1="56" x2="56" y2="200" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
	<line x1="200" y1="200" x2="56" y2="56" stroke="currentColor" stroke-linecap="round"
		stroke-linejoin="round" stroke-width="16" />
</svg>
"#;
