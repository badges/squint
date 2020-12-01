#[cfg(test)]
pub(crate) static TEST_BYTES: &[u8] = br##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="80" height="20" role="img" aria-label="npm: v3.3.0">
    <title>npm: v3.3.0</title>
    <linearGradient id="s" x2="0" y2="100%">
        <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
        <stop offset="1" stop-opacity=".1"/>
    </linearGradient>
    <clipPath id="r">
        <rect width="80" height="20" rx="3" fill="#fff"/>
    </clipPath>
    <g clip-path="url(#r)">
        <rect width="35" height="20" fill="#555"/>
        <rect x="35" width="45" height="20" fill="#007ec6"/>
        <rect width="80" height="20" fill="url(#s)"/>
    </g>
    <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">
        <text aria-hidden="true" x="185" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="250">npm</text>
        <text x="185" y="140" transform="scale(.1)" fill="#fff" textLength="250">npm</text>
        <text aria-hidden="true" x="565" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="350">v3.3.0</text>
        <text x="565" y="140" transform="scale(.1)" fill="#fff" textLength="350">v3.3.0</text>
    </g>
</svg>"##;

#[cfg(test)]
pub(crate) static LETTER_SPACING_TEST_BYTES: &[u8] = br##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="80" height="20" role="img" aria-label="npm: v3.3.0">
    <title>npm: v3.3.0</title>
    <linearGradient id="s" x2="0" y2="100%">
        <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
        <stop offset="1" stop-opacity=".1"/>
    </linearGradient>
    <clipPath id="r">
        <rect width="80" height="20" rx="3" fill="#fff"/>
    </clipPath>
    <g clip-path="url(#r)">
        <rect width="35" height="20" fill="#555"/>
        <rect x="35" width="45" height="20" fill="#007ec6"/>
        <rect width="80" height="20" fill="url(#s)"/>
    </g>
    <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">
        <text aria-hidden="true" x="185" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" letter-spacing="12.5">npm</text>
        <text x="185" y="140" transform="scale(.1)" fill="#fff" letter-spacing="12.5">npm</text>
        <text aria-hidden="true" x="565" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" letter-spacing="12.5">v3.3.0</text>
        <text x="565" y="140" transform="scale(.1)" fill="#fff" letter-spacing="12.5">v3.3.0</text>
    </g>
</svg>"##;
