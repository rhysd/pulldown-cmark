// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn math_test_1() {
    let original = r##"This sentence uses `$` delimiters to show math inline: $\sqrt{3x-1}+(1+x)^2$
$\sum_{k=1}^n a_k b_k$: Mathematical expression at head of line

`\` may follow just after the first `$`: $\{1, 2, 3\}$
"##;
    let expected = r##"<p>This sentence uses <code>$</code> delimiters to show math inline: <span class="math inline">\sqrt{3x-1}+(1+x)^2</span>
<span class="math inline">\sum_{k=1}^n a_k b_k</span>: Mathematical expression at head of line</p>
<p><code>\</code> may follow just after the first <code>$</code>: <span class="math inline">\{1, 2, 3\}</span>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_2() {
    let original = r##"**The Cauchy-Schwarz Inequality**

$$\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)$$
"##;
    let expected = r##"<p><strong>The Cauchy-Schwarz Inequality</strong></p>
<p><span class="math display">\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_3() {
    let original = r##"Oops empty $$ expression.

$$$$
"##;
    let expected = r##"<p>Oops empty $$ expression.</p>
<p><span class="math display"></span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_4() {
    let original = r##"$a<b>c</b>$

$${a*b*c} _c_ d$$

$not `code`$

$![not an](/image)$

$<https://not.a.link/>$

$&alpha;$
"##;
    let expected = r##"<p><span class="math inline">a&lt;b&gt;c&lt;/b&gt;</span></p>
<p><span class="math display">{a*b*c} _c_ d</span></p>
<p><span class="math inline">not `code`</span></p>
<p><span class="math inline">![not an](/image)</span></p>
<p><span class="math inline">&lt;https://not.a.link/&gt;</span></p>
<p><span class="math inline">&amp;alpha;</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_5() {
    let original = r##"Hello $world.

Dollar at end of line$
"##;
    let expected = r##"<p>Hello $world.</p>
<p>Dollar at end of line$</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_6() {
    let original = r##"$5x + 2 =
17$

$$\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right)
\left( \sum_{k=1}^n b_k^2 \right)$$
"##;
    let expected = r##"<p><span class="math inline">5x + 2 =
17</span></p>
<p><span class="math display">\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right)
\left( \sum_{k=1}^n b_k^2 \right)</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_7() {
    let original = r##"$not a\
hard break  
either$
"##;
    let expected = r##"<p><span class="math inline">not a\
hard break  
either</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_8() {
    let original = r##"$\$$

$$y = \$ x$$
"##;
    let expected = r##"<p><span class="math inline">\$</span></p>
<p><span class="math display">y = \$ x</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_9() {
    let original = r##"$x $ x$

$$ $ $$

$$ $$ $$
"##;
    let expected = r##"<p>$x $ x$</p>
<p><span class="math display"> $ </span></p>
<p><span class="math display"> </span> $$</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_10() {
    let original = r##"these are not math texts: $ y=x$, $y=x $, $
y=x$ and $y=x
$
"##;
    let expected = r##"<p>these are not math texts: $ y=x$, $y=x $, $
y=x$ and $y=x
$</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_11() {
    let original = r##"these are math texts: foo$y=x$bar and $y=x$bar and foo$y=x$ bar
"##;
    let expected = r##"<p>these are math texts: foo<span class="math inline">y=x</span>bar and <span class="math inline">y=x</span>bar and foo<span class="math inline">y=x</span> bar</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_12() {
    let original = r##"math texts: $x=y$! and $x=y$? and $x=y$: and $x=y$. and $x=y$"

also math texts: !$x=y$! and ?$x=y$? and :$x=y$: and .$x=y$. and "$x=y$"

braces: ($x=y$) [$x=y$] {$x=y$}
"##;
    let expected = r##"<p>math texts: <span class="math inline">x=y</span>! and <span class="math inline">x=y</span>? and <span class="math inline">x=y</span>: and <span class="math inline">x=y</span>. and <span class="math inline">x=y</span>&quot;</p>
<p>also math texts: !<span class="math inline">x=y</span>! and ?<span class="math inline">x=y</span>? and :<span class="math inline">x=y</span>: and .<span class="math inline">x=y</span>. and &quot;<span class="math inline">x=y</span>&quot;</p>
<p>braces: (<span class="math inline">x=y</span>) [<span class="math inline">x=y</span>] {<span class="math inline">x=y</span>}</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_13() {
    let original = r##"$x=y$
"##;
    let expected = r##"<p><span class="math inline">x=y</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_14() {
    let original = r##"$a$$b$

$a$$$b$$

$$a$$$b$

$$a$$$$b$$
"##;
    let expected = r##"<p><span class="math inline">a</span><span class="math inline">b</span></p>
<p><span class="math inline">a</span><span class="math display">b</span></p>
<p><span class="math display">a</span><span class="math inline">b</span></p>
<p><span class="math display">a</span><span class="math display">b</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_15() {
    let original = r##"$Inline `first$ then` code

`Code $first` then$ inline

$$ Display `first $$ then` code

`Code $$ first` then $$ display
"##;
    let expected = r##"<p><span class="math inline">Inline `first</span> then` code</p>
<p><code>Code $first</code> then$ inline</p>
<p><span class="math display"> Display `first </span> then` code</p>
<p><code>Code $$ first</code> then $$ display</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_16() {
    let original = r##"$x + y - z$

$x + y
- z$

$$ x + y
> z $$
"##;
    let expected = r##"<p><span class="math inline">x + y - z</span></p>
<p>$x + y</p>
<ul>
<li>z$</li>
</ul>
<p>$$ x + y</p>
<blockquote>
<p>z $$</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_17() {
    let original = r##"$not

math$

$$
not

math
$$
"##;
    let expected = r##"<p>$not</p>
<p>math$</p>
<p>$$
not</p>
<p>math
$$</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_18() {
    let original = r##"This is display math:
$$
\text{Hello $x^2$}
$$
And this is inline math:
$\text{Hello $x$ there!}$
"##;
    let expected = r##"<p>This is display math:
<span class="math display">
\text{Hello $x^2$}
</span>
And this is inline math:
<span class="math inline">\text{Hello $x$ there!}</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_19() {
    let original = r##"This is not valid math: $}{$

Neither is this: { $}{$ }

This is: $\}\{$

This is: $\}$

Math environment contains 2+2: $}$2+2$
"##;
    let expected = r##"<p>This is not valid math: $}{$</p>
<p>Neither is this: { $}{$ }</p>
<p>This is: <span class="math inline">\}\{</span></p>
<p>This is: <span class="math inline">\}</span></p>
<p>Math environment contains 2+2: $}<span class="math inline">2+2</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_20() {
    let original = r##"This is not display math. It is inline math:

$$\text{first $$ second}$

$$$\text{first $$ second}$

This is display math:

$$\text{first $$ second}$$

$$$\text{first $$ second}$$

This is also display math, but (counterintuitively) it's allowed to be empty
and expected to be as short as possible:

$$$$\text{first $$ second}$$
"##;
    let expected = r##"<p>This is not display math. It is inline math:</p>
<p>$<span class="math inline">\text{first $$ second}</span></p>
<p>$$<span class="math inline">\text{first $$ second}</span></p>
<p>This is display math:</p>
<p><span class="math display">\text{first $$ second}</span></p>
<p>$<span class="math display">\text{first $$ second}</span></p>
<p>This is also display math, but (counterintuitively) it's allowed to be empty
and expected to be as short as possible:</p>
<p><span class="math display"></span>\text{first $$ second}$$</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_21() {
    let original = r##"$x$ $`y`$
"##;
    let expected = r##"<p><span class="math inline">x</span> <span class="math inline">`y`</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_22() {
    let original = r##"- $a$

  ```math
  a
  ```

  $$
  a
  $$

- ```math
  b
  ```

  $$
  b
  $$
"##;
    let expected = r##"<ul>
<li>
<p><span class="math inline">a</span></p>
<pre><code class="language-math">a
</code></pre>
<p><span class="math display">a
</span></p>
</li>
<li>
<pre><code class="language-math">b
</code></pre>
<p><span class="math display">b
</span></p>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_23() {
    let original = r##"- ![node logo](https://nodejs.org/static/images/logo.svg)
- $x$
"##;
    let expected = r##"<ul>
<li><img alt="node logo" src="https://nodejs.org/static/images/logo.svg"></li>
<li><span class="math inline">x</span></li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_24() {
    let original = r##"<details>

$A = 5$

$$
A = 5
$$

</details>
"##;
    let expected = r##"<details>
<p><span class="math inline">A = 5</span></p>
<p><span class="math display">
A = 5
</span></p>
</details>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_25() {
    let original = r##"$a<b$

$$a<b$$
"##;
    let expected = r##"<p><span class="math inline">a&lt;b</span></p>
<p><span class="math display">a&lt;b</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_26() {
    let original = r##"[^a]

[^a]: Lorem $a$
"##;
    let expected = r##"<p><sup class="footnote-reference"><a href="#a">1</a></sup></p>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
<p>Lorem <span class="math inline">a</span></p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_27() {
    let original = r##"[$a$](x)
"##;
    let expected = r##"<p>
<a href="x"><span class="math inline">a</span></a>
</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_28() {
    let original = r##"a$x$

-$x$

1$x$
"##;
    let expected = r##"<p>a<span class="math inline">x</span></p>
<p>-<span class="math inline">x</span></p>
<p>1<span class="math inline">x</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_29() {
    let original = r##"_$a$ equals $b$_

_$a$ equals $b$_

**$a$ equals $b$**
"##;
    let expected = r##"<p><em><span class="math inline">a</span> equals <span class="math inline">b</span></em></p>
<p><em><span class="math inline">a</span> equals <span class="math inline">b</span></em></p>
<p><strong><span class="math inline">a</span> equals <span class="math inline">b</span></strong></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_30() {
    let original = r##"$$
a
$$

- $$
  \text{$b$}
  $$
"##;
    let expected = r##"<p><span class="math display">a</span>
</p><ul>
<li><span class="math display">\text{$b$}</span></li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_31() {
    let original = r##"$\{a\,b\}$
"##;
    let expected = r##"<p><span class="math inline">\{a\,b\}</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_32() {
    let original = r##"$a <b > c$

$[(a+b)c](d+e)$

${a}_b c_{d}$
"##;
    let expected = r##"<p><span class="math inline">a &lt;b &gt; c</span></p>
<p><span class="math inline">[(a+b)c](d+e)</span></p>
<p><span class="math inline">{a}_b c_{d}</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_33() {
    let original = r##"When $a \ne 0$, there are two solutions to $(ax^2 + bx + c = 0)$ and they are
$$ x = {-b \pm \sqrt{b^2-4ac} \over 2a} $$
"##;
    let expected = r##"<p>When <span class="math inline">a \ne 0</span>, there are two solutions to <span class="math inline">(ax^2 + bx + c = 0)</span> and they are
<span class="math display"> x = {-b \pm \sqrt{b^2-4ac} \over 2a} </span>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_34() {
    let original = r##"$x = \$$
"##;
    let expected = r##"<p><span class="math inline">x = \$</span></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_35() {
    let original = r##"_Equation $\Omega(69)$ in italic text_
"##;
    let expected = r##"<p><em>Equation <span class="math inline">\Omega(69)</span> in italic text</em></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_36() {
    let original = r##"$\pi$
'$\pi$
"$\pi$
($\pi$
[$\pi$
{$\pi$
/$\pi$
"##;
    let expected = r##"<p>
<span class="math inline">\pi</span>
'<span class="math inline">\pi</span>
"<span class="math inline">\pi</span>
(<span class="math inline">\pi</span>
[<span class="math inline">\pi</span>
{<span class="math inline">\pi</span>
/<span class="math inline">\pi</span>
</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_37() {
    let original = r##"| first $|$ second |
|--------|---------|
| a ${   | }$ b    |
"##;
    let expected = r##"<table><thead>
<tr><th>first $</th><th>$ second</th></tr>
</thead><tbody>
<tr><td>a ${</td><td>}$ b</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_38() {
    let original = r##"| first $\|$ second |
|-------------------|
| a ${   \| }$ b    |
"##;
    let expected = r##"<table><thead>
<tr><th>first <span class="math inline">|</span> second</th></tr>
</thead><tbody>
<tr><td>a <span class="math inline">{   | }</span> b</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn math_test_39() {
    let original = r##"| Description | Test case |
|-------------|-----------|
| Single      | $\$       |
| Double      | $\\$      |
| Basic test  | $\|$      |
| Basic test 2| $\|\|\$   |
| Basic test 3| $x\|y\|z\$|
| Not pipe    | $\.$      |
| Combo       | $\.\|$    |
| Combo 2     | $\.\|\$   |
| Extra       | $\\\.$    |
| Wait, what? | $\\|$     |
| Wait, what? | $\\\|$    |
| Wait, what? | $\\\\|$   |
| Wait, what? | $\\\\\|$  |
"##;
    let expected = r##"<table><thead><tr><th>Description</th><th>Test case</th></tr></thead><tbody>
<tr><td>Single</td><td>$$</td></tr>
<tr><td>Double</td><td><span class="math inline">\\</span></td></tr>
<tr><td>Basic test</td><td><span class="math inline">|</span></td></tr>
<tr><td>Basic test 2</td><td>$||$</td></tr>
<tr><td>Basic test 3</td><td>$x|y|z$</td></tr>
<tr><td>Not pipe</td><td><span class="math inline">\.</span></td></tr>
<tr><td>Combo</td><td><span class="math inline">\.|</span></td></tr>
<tr><td>Combo 2</td><td>$.|$</td></tr>
<tr><td>Extra</td><td><span class="math inline">\\\.</span></td></tr>
<tr><td>Wait, what?</td><td><span class="math inline">\|</span></td></tr>
<tr><td>Wait, what?</td><td><span class="math inline">\\|</span></td></tr>
<tr><td>Wait, what?</td><td><span class="math inline">\\\|</span></td></tr>
<tr><td>Wait, what?</td><td><span class="math inline">\\\\|</span></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}
