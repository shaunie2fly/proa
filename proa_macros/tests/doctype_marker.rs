// Copyright Proa Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use proa_core::{render_to_string, render_to_string_async};
use proa_macros::{html, html_sync, xml_sync};

#[test]
fn html_sync_omits_proa_doctype_marker() {
    let output = render_to_string(html_sync! {
        <!DOCTYPE html>
        <html lang="en">
            <body>"Hello"</body>
        </html>
    });

    assert_eq!(
        output,
        "<!DOCTYPE html><html lang=\"en\"><body>Hello</body></html>"
    );
}

#[tokio::test]
async fn html_omits_proa_doctype_marker() {
    let output = render_to_string_async(html! {
        <!DOCTYPE html>
        <html lang="en">
            <body>{proa_core::text("Hello")}</body>
        </html>
    })
    .await;

    assert_eq!(
        output,
        "<!DOCTYPE html><html lang=\"en\"><body>Hello</body></html>"
    );
}

#[test]
fn xml_sync_does_not_mark_doctype() {
    let output = render_to_string(xml_sync! {
        <!DOCTYPE html>
        <html>
            <body>"Hello"</body>
        </html>
    });

    assert_eq!(output, "<!DOCTYPE html><html><body>Hello</body></html>");
}
