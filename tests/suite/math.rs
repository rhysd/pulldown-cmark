// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn math_test_1() {
    let original = r##"This sentence uses `$` delimiters to show math inline:  $\sqrt{3x-1}+(1+x)^2$
$\sum_{k=1}^n a_k b_k$: Mathematical expression at head of line

`\` may follow just after the first `$`: $\{1, 2, 3\}$
"##;
    let expected = r##"<p>This sentence uses <code>$</code> delimiters to show math inline:  <span class="math inline">\sqrt{3x-1}+(1+x)^2</span>
<span class="math inline">\sum_{k=1}^n a_k b_k</span>: Mathematical expression at head of line</p>
<p><code>\</code> may follow just after the first <code>$</code>:  <span class="math inline">\{1, 2, 3\}</span>
"##;

    test_markdown_html(original, expected, false);
}

#[test]
fn math_test_2() {
    let original = r##"**The Cauchy-Schwarz Inequality**

$$\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)$$
"##;
    let expected = r##"<p><strong>The Cauchy-Schwarz Inequality</strong></p>
<div class="math block">\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)</div>
"##;

    test_markdown_html(original, expected, false);
}

#[test]
fn math_test_3() {
    let original = r##"Oops empty $$ expression.

$$$$

This is also empty: $``$
"##;
    let expected = r##"<p>Oops empty $$ expression.</p>
<p>$$$$</p>
<p>This is also empty: $``$</p>
"##;

    test_markdown_html(original, expected, false);
}

#[test]
fn math_test_4() {
    let original = r##"Hello $ world.

Dollar at end of line: $
"##;
    let expected = r##"<p>Hello $ world.</p>
<p>Dollar at end of line: $</p>
"##;

    test_markdown_html(original, expected, false);
}

#[test]
fn math_test_5() {
    let original = r##"$$\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right)
\left( \sum_{k=1}^n b_k^2 \right)$$
"##;
    let expected = r##"<div class="math block">\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right)
\left( \sum_{k=1}^n b_k^2 \right)</div>
"##;

    test_markdown_html(original, expected, false);
}

#[test]
fn math_test_6() {
    let original = r##"$\$$

$$y = \$ x$$
"##;
    let expected = r##"<p><span class="math inline">\$</span></p>
<div class="math block">y = \$ x</div>
"##;

    test_markdown_html(original, expected, false);
}

#[test]
fn math_test_7() {
    let original = r##"this is math inline $`y=x`$ test.
dollar can be contained: $`foo$bar`$
"##;
    let expected = r##"<p>this is math inline <span class="math inline">y=x</span> test.
dollar can be contained: <span class="math inline">foo$bar</span></p>
"##;

    test_markdown_html(original, expected, false);
}