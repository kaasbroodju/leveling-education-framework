use tidos::{Component, Page};

pub const ACCESSIBILITY_NEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M20.5 6c-2.61.7-5.67 1-8.5 1s-5.89-.3-8.5-1L3 8c1.86.5 4 .83 6 1v13h2v-6h2v6h2V9c2-.17 4.14-.5 6-1l-.5-2zM12 6c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2z"/></svg>"##;
pub const ANNOUNCEMENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 9h-2V5h2v6zm0 4h-2v-2h2v2z"/></svg>"##;
pub const BIOTECH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" enable-background="new 0 0 24 24" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M7,19c-1.1,0-2,0.9-2,2h14c0-1.1-0.9-2-2-2h-4v-2h3c1.1,0,2-0.9,2-2h-8c-1.66,0-3-1.34-3-3c0-1.09,0.59-2.04,1.46-2.56 C8.17,9.03,8,8.54,8,8c0-0.21,0.04-0.42,0.09-0.62C6.28,8.13,5,9.92,5,12c0,2.76,2.24,5,5,5v2H7z"/><path d="M10.56,5.51C11.91,5.54,13,6.64,13,8c0,0.75-0.33,1.41-0.85,1.87l0.59,1.62l0.94-0.34l0.34,0.94l1.88-0.68l-0.34-0.94 l0.94-0.34L13.76,2.6l-0.94,0.34L12.48,2L10.6,2.68l0.34,0.94L10,3.97L10.56,5.51z"/><circle cx="10.5" cy="8" r="1.5"/></g></g></svg>"##;
pub const BUILD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M705-128 447-388q-23 8-46 13t-47 5q-97 0-165-67.5T121-602q0-31 8-60.5t23-55.5l145 145 92-86-149-149q26-15 55-23.5t59-8.5q99 0 168.5 69.5T592-602q0 24-5 47t-13 46l259 258q11 11 11 26.5T833-198l-76 70q-11 11-26 11t-26-11Z"/></svg>"##;
pub const CALENDAR_MONTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" enable-background="new 0 0 20 20" height="48px" viewBox="0 0 20 20" width="48px" fill="currentColor"><g><rect fill="none" height="20" width="20"/></g><g><path d="M15.5,4H14V2h-1.5v2h-5V2H6v2H4.5C3.67,4,3,4.68,3,5.5v11C3,17.32,3.67,18,4.5,18h11c0.83,0,1.5-0.68,1.5-1.5v-11 C17,4.68,16.33,4,15.5,4z M15.5,16.5h-11V9h11V16.5z M7.5,12H6v-1.5h1.5V12z M10.75,12h-1.5v-1.5h1.5V12z M14,12h-1.5v-1.5H14V12z M7.5,15H6v-1.5h1.5V15z M10.75,15h-1.5v-1.5h1.5V15z M14,15h-1.5v-1.5H14V15z"/></g></svg>"##;
pub const CATEGORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 0 24 24" width="24px" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2l-5.5 9h11z"/><circle cx="17.5" cy="17.5" r="4.5"/><path d="M3 13.5h8v8H3z"/></svg>"##;
pub const CHAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 9h12v2H6V9zm8 5H6v-2h8v2zm4-6H6V6h12v2z"/></svg>"##;
pub const CO_PRESENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" enable-background="new 0 0 20 20" height="48px" viewBox="0 0 20 20" width="48px" fill="currentColor"><g><rect fill="none" height="20" width="20"/></g><g><g><path d="M17.5,3h-15C1.67,3,1,3.67,1,4.5V12h1.5V4.5h15V17c0.83,0,1.5-0.67,1.5-1.5v-11C19,3.67,18.33,3,17.5,3z"/><circle cx="7" cy="9" r="3"/><path d="M12.03,14.37C10.56,13.5,8.84,13,7,13s-3.56,0.5-5.03,1.37C1.36,14.72,1,15.39,1,16.09V18h12v-1.91 C13,15.39,12.64,14.72,12.03,14.37z"/></g></g></svg>"##;
pub const DESIGN_SERVICES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="m357-513 90-90-75-75-48 48-42-42 48-48-75-74-90 90 192 191Zm346 348 90-91-75-75-48 48-42-42 48-48-74-74-90 90 191 192Zm-87-520 70 70 94-94-70-70-94 94ZM276-120H120v-156l194-194L80-704l174-176 236 235 178-178q9-9 20-13t22-4q11 0 22 4t20 13l71 71q9 9 13 20t4 22q0 11-4 22t-13 20L645-490l235 235L705-81 471-315 276-120Z"/></svg>"##;
pub const DIRECTIONS_RUN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.6 13.9l1-4.4 2.1 2v6h2v-7.5l-2.1-2 .6-3c1.3 1.5 3.3 2.5 5.5 2.5v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1l-5.2 2.2v4.7h2v-3.4l1.8-.7-1.6 8.1-4.9-1-.4 2 7 1.4z"/></svg>"##;
pub const DOMAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" enable-background="new 0 0 24 24" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><g><path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,7V3H2v18h20V7H12z M6,19H4v-2h2V19z M6,15H4v-2h2V15z M6,11H4V9h2V11z M6,7H4V5h2V7z M10,19H8v-2h2V19z M10,15H8v-2h2 V15z M10,11H8V9h2V11z M10,7H8V5h2V7z M20,19h-8v-2h2v-2h-2v-2h2v-2h-2V9h8V19z M18,11h-2v2h2V11z M18,15h-2v2h2V15z"/></g></svg>"##;
pub const EXPLORE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="m303-303 270-83 83-270-270 83-83 270Zm177-137q-17 0-28.5-11.5T440-480q0-17 11.5-28.5T480-520q17 0 28.5 11.5T520-480q0 17-11.5 28.5T480-440Zm0 360q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 31.5-156t86-127Q252-817 325-848.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 82-31.5 155T763-197.5q-54 54.5-127 86T480-80Z"/></svg>"##;
pub const FACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 0 24 24" width="24px" fill="currentColor"><path d="M0 0h24v24H0z" fill="none"/><path d="M9 11.75c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zm6 0c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8 0-.29.02-.58.05-.86 2.36-1.05 4.23-2.98 5.21-5.37C11.07 8.33 14.05 10 17.42 10c.78 0 1.53-.09 2.25-.26.21.71.33 1.47.33 2.26 0 4.41-3.59 8-8 8z"/></svg>"##;

pub const HANDSHAKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" enable-background="new 0 0 20 20" height="48px" viewBox="0 0 20 20" width="48px" fill="currentColor"><g><rect fill="none" height="20" width="20"/></g><g><path d="M17.62,10.17c1.17-1.17,1.17-3.07,0-4.24l-2.55-2.55c-1.17-1.17-3.07-1.17-4.24,0l-0.27,0.27l3.59,3.58 c0.62,0.62,0.62,1.64,0,2.26c-0.62,0.62-1.64,0.62-2.26,0l-3-3l-4.55,4.55c-0.31,0.31-0.31,0.82,0,1.13c0.31,0.31,0.82,0.31,1.13,0 l3.79-3.79l0.57,0.57l-3.79,3.79c-0.31,0.31-0.31,0.82,0,1.13c0.31,0.31,0.82,0.31,1.13,0l3.79-3.79l0.57,0.57l-3.79,3.79 c-0.31,0.31-0.31,0.82,0,1.13c0.31,0.31,0.82,0.31,1.13,0l3.79-3.79l0.57,0.57l-3.79,3.79c-0.31,0.31-0.31,0.82,0,1.13 c0.31,0.31,0.82,0.31,1.13,0L17.62,10.17z M4.48,4.87 M2.38,5.91c-1.17,1.17-1.17,3.07,0,4.24L3.23,11l5.64-5.64l3.57,3.57 c0.31,0.31,0.83,0.31,1.14,0c0.31-0.31,0.31-0.82,0-1.13L9.15,3.38c-1.17-1.17-3.07-1.17-4.24,0L2.38,5.91z"/></g></svg>"##;
pub const INFO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M453-280h60v-240h-60v240Zm50.5-323.2q9.5-9.2 9.5-22.8 0-14.45-9.48-24.22-9.48-9.78-23.5-9.78t-23.52 9.78Q447-640.45 447-626q0 13.6 9.48 22.8 9.48 9.2 23.5 9.2t23.52-9.2ZM480.27-80q-82.74 0-155.5-31.5Q252-143 197.5-197.5t-86-127.34Q80-397.68 80-480.5t31.5-155.66Q143-709 197.5-763t127.34-85.5Q397.68-880 480.5-880t155.66 31.5Q709-817 763-763t85.5 127Q880-563 880-480.27q0 82.74-31.5 155.5Q817-252 763-197.68q-54 54.31-127 86Q563-80 480.27-80Z"/></svg>"##;
pub const LAPTOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M140-240q-24 0-42-18t-18-42v-480q0-24 18-42t42-18h680q24 0 42 18t18 42v480q0 24-18 42t-42 18H140ZM40-120v-60h880v60H40Z"/></svg>"##;
pub const MEMORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M364-364v-231h231v231H364ZM353-99v-86h-95q-28.73 0-50.86-22.14Q185-229.27 185-258v-95H99v-73h86v-112H99v-73h86v-95q0-28.72 22.14-50.86Q229.27-779 258-779h95v-82h73v82h112v-82h73v82h95q28.72 0 50.86 22.14T779-706v95h82v73h-82v112h82v73h-82v95q0 28.73-22.14 50.86Q734.72-185 706-185h-95v86h-73v-86H426v86h-73Zm353-159v-448H258v448h448Z"/></svg>"##;
pub const MENU_BOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M560-574v-48q33-14 67.5-21t72.5-7q26 0 51 4t49 10v44q-24-9-48.5-13.5T700-610q-38 0-73 9.5T560-574Zm0 220v-49q33-13.5 67.5-20.25T700-430q26 0 51 4t49 10v44q-24-9-48.5-13.5T700-390q-38 0-73 9t-67 27Zm0-110v-48q33-14 67.5-21t72.5-7q26 0 51 4t49 10v44q-24-9-48.5-13.5T700-500q-38 0-73 9.5T560-464Zm-48 214q50-25 98-37.5T712-300q38 0 78.5 6t69.5 16v-429q-34-17-71.82-25-37.82-8-76.18-8-54 0-104.5 16.5T512-677v427Zm-30 90q-51-38-111-58.5T248-239q-36.54 0-71.77 9T106-208q-23.1 11-44.55-3Q40-225 40-251v-463q0-15 7-27.5T68-761q42-20 87.5-29.5T248-800q63 0 122.5 17T482-731q51-35 109.5-52T712-800q47.18 0 92.09 9.5Q849-781 891-761q14 7 21.5 19.5T920-714v463q0 27.89-22.5 42.45Q875-194 853-208q-34-14-69.23-22.5Q748.54-239 712-239q-63 0-121 21t-109 58Z"/></svg>"##;
pub const OPEN_IN_NEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M180-120q-24 0-42-18t-18-42v-600q0-24 18-42t42-18h279v60H180v600h600v-279h60v279q0 24-18 42t-42 18H180Zm202-219-42-43 398-398H519v-60h321v321h-60v-218L382-339Z"/></svg>"##;
pub const PACKAGE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M450-85v-378L120-654v344q0 16 8 30t22 22L450-85Zm60 0 300-173q14-8 22-22t8-30v-345L510-463v378Zm164-542 133-77-297-171q-14-8-30-8t-30 8l-104 60 328 188ZM480-514l133-78-328-188-132 77 327 189Z"/></svg>"##;
pub const PSYCHOLOGY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M449-374h60l3-44q12-2 22.5-8.5T553-441l42 14 28-48-30-24q5-14 5-29t-5-29l30-24-28-48-42 14q-8-8-19-14t-22-9l-3-44h-60l-3 44q-11 3-22 9t-19 14l-42-14-28 48 30 24q-5 14-5 29t5 29l-30 24 28 48 42-14q8 8 18.5 14.5T446-418l3 44Zm-19.5-104.5Q409-499 409-528t20.5-49.5Q450-598 479-598t49.5 20.5Q549-557 549-528t-20.5 49.5Q508-458 479-458t-49.5-20.5ZM240-80v-172q-57-52-88.5-121.5T120-520q0-150 105-255t255-105q125 0 221.5 73.5T827-615l55 218q4 14-5 25.5T853-360h-93v140q0 25-17.5 42.5T700-160H600v80H240Z"/></svg>"##;
pub const SCHOOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M860-283v-282L479-360 40-600l439-240 441 240v317h-60ZM479-120 189-279v-210l290 159 290-159v210L479-120Z"/></svg>"##;
pub const SMART_BUTTON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" enable-background="new 0 0 24 24" height="48px" viewBox="0 0 24 24" width="48px" fill="currentColor"><g><rect fill="none" height="24" width="24"/><path d="M22,9v6c0,1.1-0.9,2-2,2h-1l0-2h1V9H4v6h6v2H4c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h16C21.1,7,22,7.9,22,9z M14.5,19 l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09L14.5,19z M17,14l0.62-1.38L19,12l-1.38-0.62L17,10l-0.62,1.38 L15,12l1.38,0.62L17,14z M14.5,19l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09L14.5,19z M17,14l0.62-1.38 L19,12l-1.38-0.62L17,10l-0.62,1.38L15,12l1.38,0.62L17,14z"/></g></svg>"##;
pub const STORAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M120-160v-148h720v148H120Zm60-38h72v-72h-72v72Zm-60-454v-148h720v148H120Zm60-38h72v-72h-72v72Zm-60 284v-148h720v148H120Zm60-38h72v-72h-72v72Z"/></svg>"##;
pub const TUNE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="48px" viewBox="0 -960 960 960" width="48px" fill="currentColor"><path d="M427-120v-225h60v83h353v60H487v82h-60Zm-307-82v-60h247v60H120Zm187-166v-82H120v-60h187v-84h60v226h-60Zm120-82v-60h413v60H427Zm166-165v-225h60v82h187v60H653v83h-60Zm-473-83v-60h413v60H120Z"/></svg>"##;
pub const WEB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M140-160q-24 0-42-18t-18-42v-520q0-24 18-42t42-18h680q24 0 42 18t18 42v520q0 24-18 42t-42 18H140Zm0-60h461v-163H140v163Zm521 0h159v-386H661v386ZM140-443h461v-163H140v163Z"/></svg>"##;

// ---------------------------------------------------------------------------
// Dynamic lookup — used by filter matrices that select icons at runtime via
// the domain's Icon::to_icon() method.
// ---------------------------------------------------------------------------

pub fn svg_by_name(name: &str) -> &'static str {
	match name {
		"accessibility_new" => ACCESSIBILITY_NEW_SVG,
		"announcement" => ANNOUNCEMENT_SVG,
		"biotech" => BIOTECH_SVG,
		"build" => BUILD_SVG,
		"calendar_month" => CALENDAR_MONTH_SVG,
		"category" => CATEGORY_SVG,
		"chat" => CHAT_SVG,
		"co_present" => CO_PRESENT_SVG,
		"design_services" => DESIGN_SERVICES_SVG,
		"directions_run" => DIRECTIONS_RUN_SVG,
		"domain" => DOMAIN_SVG,
		"explore" => EXPLORE_SVG,
		"face" => FACE_SVG,
		"handshake" => HANDSHAKE_SVG,
		"info" => INFO_SVG,
		"laptop" => LAPTOP_SVG,
		"memory" => MEMORY_SVG,
		"menu_book" => MENU_BOOK_SVG,
		"open_in_new" => OPEN_IN_NEW_SVG,
		"package_2" => PACKAGE_2_SVG,
		"psychology" => PSYCHOLOGY_SVG,
		"school" => SCHOOL_SVG,
		"smart_button" => SMART_BUTTON_SVG,
		"storage" => STORAGE_SVG,
		"tune" => TUNE_SVG,
		"web" => WEB_SVG,
		_ => "",
	}
}

// ---------------------------------------------------------------------------
// Component structs — zero-prop unit structs, one per icon.
// Use these for static icon placements: <InfoIcon />
// ---------------------------------------------------------------------------

pub struct AccessibilityNewIcon;
pub struct AnnouncementIcon;
pub struct BiotechIcon;
pub struct BuildIcon;
pub struct CalendarMonthIcon;
pub struct CategoryIcon;
pub struct ChatIcon;
pub struct CoPresentIcon;
pub struct DesignServicesIcon;
pub struct DirectionsRunIcon;
pub struct DomainIcon;
pub struct ExploreIcon;
pub struct FaceIcon;
pub struct HandshakeIcon;
pub struct InfoIcon;
pub struct LaptopIcon;
pub struct MemoryIcon;
pub struct MenuBookIcon;
pub struct OpenInNewIcon;
pub struct Package2Icon;
pub struct PsychologyIcon;
pub struct SchoolIcon;
pub struct SmartButtonIcon;
pub struct StorageIcon;
pub struct TuneIcon;
pub struct WebIcon;

impl Component for AccessibilityNewIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		ACCESSIBILITY_NEW_SVG.to_string()
	}
}
impl Component for AnnouncementIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		ANNOUNCEMENT_SVG.to_string()
	}
}
impl Component for BiotechIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		BIOTECH_SVG.to_string()
	}
}
impl Component for BuildIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		BUILD_SVG.to_string()
	}
}
impl Component for CalendarMonthIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		CALENDAR_MONTH_SVG.to_string()
	}
}
impl Component for CategoryIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		CATEGORY_SVG.to_string()
	}
}
impl Component for ChatIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		CHAT_SVG.to_string()
	}
}
impl Component for CoPresentIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		CO_PRESENT_SVG.to_string()
	}
}
impl Component for DesignServicesIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		DESIGN_SERVICES_SVG.to_string()
	}
}
impl Component for DirectionsRunIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		DIRECTIONS_RUN_SVG.to_string()
	}
}
impl Component for DomainIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		DOMAIN_SVG.to_string()
	}
}
impl Component for ExploreIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		EXPLORE_SVG.to_string()
	}
}
impl Component for FaceIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		FACE_SVG.to_string()
	}
}
impl Component for HandshakeIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		HANDSHAKE_SVG.to_string()
	}
}
impl Component for InfoIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		INFO_SVG.to_string()
	}
}
impl Component for LaptopIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		LAPTOP_SVG.to_string()
	}
}
impl Component for MemoryIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		MEMORY_SVG.to_string()
	}
}
impl Component for MenuBookIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		MENU_BOOK_SVG.to_string()
	}
}
impl Component for OpenInNewIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		OPEN_IN_NEW_SVG.to_string()
	}
}
impl Component for Package2Icon {
	fn to_render(&self, _page: &mut Page) -> String {
		PACKAGE_2_SVG.to_string()
	}
}
impl Component for PsychologyIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		PSYCHOLOGY_SVG.to_string()
	}
}
impl Component for SchoolIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		SCHOOL_SVG.to_string()
	}
}
impl Component for SmartButtonIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		SMART_BUTTON_SVG.to_string()
	}
}
impl Component for StorageIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		STORAGE_SVG.to_string()
	}
}
impl Component for TuneIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		TUNE_SVG.to_string()
	}
}
impl Component for WebIcon {
	fn to_render(&self, _page: &mut Page) -> String {
		WEB_SVG.to_string()
	}
}
