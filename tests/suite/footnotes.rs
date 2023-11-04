// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn footnotes_test_1() {
    let original = r##"Lorem ipsum.[^a] [^missing]

[^a]: Cool.
"##;
    let expected = r##"<p>Lorem ipsum.<sup class="footnote-reference"><a href="#a">1</a></sup> [^missing]</p>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
<p>Cool.</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_2() {
    let original = r##"> This is the song that never ends.\
> Yes it goes on and on my friends.[^lambchops]
>
> [^lambchops]: <https://www.youtube.com/watch?v=0U2zJOryHKQ>
"##;
    let expected = r##"<blockquote>
<p>This is the song that never ends.<br />
Yes it goes on and on my friends.<sup class="footnote-reference"><a href="#lambchops">1</a></sup></p>
<div class="footnote-definition" id="lambchops"><sup class="footnote-definition-label">1</sup>
<p><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">https://www.youtube.com/watch?v=0U2zJOryHKQ</a></p>
</div>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_3() {
    let original = r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
 * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
 * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
 * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)
"##;
    let expected = r##"<p>Songs that simply loop are a popular way to annoy people. <sup class="footnote-reference"><a href="#examples">1</a></sup></p>
<div class="footnote-definition" id="examples"><sup class="footnote-definition-label">1</sup></div>
<ul>
<li><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">The song that never ends</a></li>
<li><a href="https://www.youtube.com/watch?v=TehWI09qxls">I know a song that gets on everybody's nerves</a></li>
<li><a href="https://www.youtube.com/watch?v=qVjCag8XoHQ">Ninety-nine bottles of beer on the wall</a></li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_4() {
    let original = r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
    * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
    * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)
"##;
    let expected = r##"<p>Songs that simply loop are a popular way to annoy people. <sup class="footnote-reference"><a href="#examples">1</a></sup></p>
<div class="footnote-definition" id="examples"><sup class="footnote-definition-label">1</sup>
<ul>
<li><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">The song that never ends</a></li>
<li><a href="https://www.youtube.com/watch?v=TehWI09qxls">I know a song that gets on everybody's nerves</a></li>
<li><a href="https://www.youtube.com/watch?v=qVjCag8XoHQ">Ninety-nine bottles of beer on the wall</a></li>
</ul>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_5() {
    let original = r##"[^not-code] [^code] [^quote] [^not-quote] [^indented-quote]

[^not-code]:         not code

[^code]:
        code

[^quote]: > quote

[^not-quote]:
 > external quote

[^indented-quote]:
    > indented quote
"##;
    let expected = r##"<p><sup class="footnote-reference"><a href="#not-code">1</a></sup> <sup class="footnote-reference"><a href="#code">2</a></sup> <sup class="footnote-reference"><a href="#quote">3</a></sup> <sup class="footnote-reference"><a href="#not-quote">4</a></sup> <sup class="footnote-reference"><a href="#indented-quote">5</a></sup></p>
<div class="footnote-definition" id="not-code"><sup class="footnote-definition-label">1</sup>
<p>not code</p>
</div>
<div class="footnote-definition" id="code"><sup class="footnote-definition-label">2</sup>
<pre><code>code
</code></pre>
</div>
<div class="footnote-definition" id="quote"><sup class="footnote-definition-label">3</sup>
<blockquote><p>quote</p></blockquote>
</div>
<div class="footnote-definition" id="not-quote"><sup class="footnote-definition-label">4</sup></div>
<blockquote><p>external quote</p></blockquote><div class="footnote-definition" id="indented-quote"><sup class="footnote-definition-label">5</sup>
<blockquote><p>indented quote</p></blockquote>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_6() {
    let original = r##"[^ab] [^cd]

[^ab]: a
b

[^cd]: c\
d
"##;
    let expected = r##"<p><sup class="footnote-reference"><a href="#ab">1</a></sup> <sup class="footnote-reference"><a href="#cd">2</a></sup></p>
<div class="footnote-definition" id="ab"><sup class="footnote-definition-label">1</sup>
<p>a
b</p>
</div>
<div class="footnote-definition" id="cd"><sup class="footnote-definition-label">2</sup>
<p>c<br>
d</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_7() {
    let original = r##"[^lorem]: If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.

[^ipsum]: If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

    I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.
"##;
    let expected = r##"<div class="footnote-definition" id="lorem"><sup class="footnote-definition-label">1</sup>
<p>If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.</p>
</div>
<p>I had largely given over my inquiries into what Professor Angell called the &quot;Cthulhu Cult&quot;, and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.</p>
<div class="footnote-definition" id="ipsum"><sup class="footnote-definition-label">2</sup>
<p>If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.</p>
<p>I had largely given over my inquiries into what Professor Angell called the &quot;Cthulhu Cult&quot;, and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_8() {
    let original = r##"[^ipsum]: How much wood would a woodchuck chuck.

If a woodchuck could chuck wood.


# Forms of entertainment that aren't childish
"##;
    let expected = r##"<div class="footnote-definition" id="ipsum"><sup class="footnote-definition-label">1</sup>
<p>How much wood would a woodchuck chuck.</p>
</div>
<p>If a woodchuck could chuck wood.</p>
<h1>Forms of entertainment that aren't childish</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_9() {
    let original = r##"Footnotes [^one] [^many].

[^one]:





    first paragraph inside footnote

[^many]: first paragraph inside footnote





    second paragraph still inside footnote
"##;
    let expected = r##"<p>Footnotes <sup class="footnote-reference"><a href="#one">1</a></sup> <sup class="footnote-reference"><a href="#many">2</a></sup>.</p>
<div class="footnote-definition" id="one"><sup class="footnote-definition-label">1</sup>
<p>first paragraph inside footnote</p>
</div>
<div class="footnote-definition" id="many"><sup class="footnote-definition-label">2</sup>
<p>first paragraph inside footnote</p>
<p>second paragraph still inside footnote</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_10() {
    let original = r##"> He's also really stupid. [^why]
>
> [^why]: Because your mamma!

As such, we can guarantee that the non-childish forms of entertainment are probably more entertaining to adults, since, having had a whole childhood doing the childish ones, the non-childish ones are merely the ones that haven't gotten boring yet.
"##;
    let expected = r##"<blockquote>
<p>He's also really stupid. <sup class="footnote-reference"><a href="#why">1</a></sup></p>
<div class="footnote-definition" id="why"><sup class="footnote-definition-label">1</sup>
<p>Because your mamma!</p>
</div>
</blockquote>
<p>As such, we can guarantee that the non-childish forms of entertainment are probably more entertaining to adults, since, having had a whole childhood doing the childish ones, the non-childish ones are merely the ones that haven't gotten boring yet.</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_11() {
    let original = r##"Nested footnotes are considered poor style. [^a] [^xkcd] [^indent1] [^indent2]

[^a]: This does not mean that footnotes cannot reference each other. [^b]

[^b]: This means that a footnote definition cannot be directly inside another footnote definition.
> This means that a footnote cannot be directly inside another footnote's body. [^e]
>
> [^e]: They can, however, be inside anything else.

[^xkcd]: [The other kind of nested footnote is, however, considered poor style.](https://xkcd.com/1208/)

[^indent1]: indent1

    [^indent2]: indent2
"##;
    let expected = r##"<p>Nested footnotes are considered poor style. <sup class="footnote-reference"><a href="#a">1</a></sup> <sup class="footnote-reference"><a href="#xkcd">2</a></sup> <sup class="footnote-reference"><a href="#indent1">3</a></sup> <sup class="footnote-reference"><a href="#indent2">4</a></sup></p>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
<p>This does not mean that footnotes cannot reference each other. <sup class="footnote-reference"><a href="#b">5</a></sup></p>
</div>
<div class="footnote-definition" id="b"><sup class="footnote-definition-label">5</sup>
<p>This means that a footnote definition cannot be directly inside another footnote definition.</p>
</div>
<blockquote>
<p>This means that a footnote cannot be directly inside another footnote's body. <sup class="footnote-reference"><a href="#e">6</a></sup></p>
<div class="footnote-definition" id="e"><sup class="footnote-definition-label">6</sup>
<p>They can, however, be inside anything else.</p>
</div>
</blockquote>
<div class="footnote-definition" id="xkcd"><sup class="footnote-definition-label">2</sup>
<p><a href="https://xkcd.com/1208/">The other kind of nested footnote is, however, considered poor style.</a></p>
</div>
<div class="footnote-definition" id="indent1"><sup class="footnote-definition-label">3</sup>
<p>indent1</p>
</div>
<div class="footnote-definition" id="indent2"><sup class="footnote-definition-label">4</sup>
<p>indent2</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_12() {
    let original = r##"[^Doh] Ray Me Fa So La Te Do! [^1]

[^Doh]: I know. Wrong Doe. And it won't render right.
[^1]: Common for people practicing music.
"##;
    let expected = r##"<p><sup class="footnote-reference"><a href="#Doh">1</a></sup> Ray Me Fa So La Te Do! <sup class="footnote-reference"><a href="#1">2</a></sup></p>
<div class="footnote-definition" id="Doh"><sup class="footnote-definition-label">1</sup>
<p>I know. Wrong Doe. And it won't render right.</p>
</div>
<div class="footnote-definition" id="1"><sup class="footnote-definition-label">2</sup>
<p>Common for people practicing music.</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_13() {
    let original = r##"Lorem ipsum.[^a]

An unordered list before the footnotes:
* Ipsum
* Lorem

[^a]: Cool.
"##;
    let expected = r##"<p>Lorem ipsum.<sup class="footnote-reference"><a href="#a">1</a></sup></p>
<p>An unordered list before the footnotes:</p>
<ul>
    <li>Ipsum</li>
    <li>Lorem</li>
</ul>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
    <p>Cool.</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_14() {
    let original = r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]: * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
* [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
* [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)


Songs that simply loop are a popular way to annoy people. [^examples2]

[^examples2]: * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ) 2
    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls) 2
    - [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ) 2


Songs that simply loop are a popular way to annoy people. [^examples3]

[^examples3]: * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ) 3

    * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls) 3

    * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ) 3
"##;
    let expected = r##"<p>Songs that simply loop are a popular way to annoy people. <sup class="footnote-reference"><a href="#examples">1</a></sup></p>
<div class="footnote-definition" id="examples"><sup class="footnote-definition-label">1</sup>
<ul>
<li><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">The song that never ends</a></li>
</ul>
</div>
<ul>
<li><a href="https://www.youtube.com/watch?v=TehWI09qxls">I know a song that gets on everybody's nerves</a></li>
<li><a href="https://www.youtube.com/watch?v=qVjCag8XoHQ">Ninety-nine bottles of beer on the wall</a></li>
</ul>
<p>Songs that simply loop are a popular way to annoy people. <sup class="footnote-reference"><a href="#examples2">2</a></sup></p>
<div class="footnote-definition" id="examples2"><sup class="footnote-definition-label">2</sup>
<ul>
<li><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">The song that never ends</a> 2</li>
<li><a href="https://www.youtube.com/watch?v=TehWI09qxls">I know a song that gets on everybody's nerves</a> 2</li>
</ul>
<ul>
<li><a href="https://www.youtube.com/watch?v=qVjCag8XoHQ">Ninety-nine bottles of beer on the wall</a> 2</li>
</ul>
</div>
<p>Songs that simply loop are a popular way to annoy people. <sup class="footnote-reference"><a href="#examples3">3</a></sup></p>
<div class="footnote-definition" id="examples3"><sup class="footnote-definition-label">3</sup>
<ul>
<li><p><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">The song that never ends</a> 3</p>
</li><li><p><a href="https://www.youtube.com/watch?v=TehWI09qxls">I know a song that gets on everybody's nerves</a> 3</p>
</li><li><p><a href="https://www.youtube.com/watch?v=qVjCag8XoHQ">Ninety-nine bottles of beer on the wall</a> 3</p></li>
</ul>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_15() {
    let original = r##"My [cmark-gfm][^c].

My [cmark-gfm][cmark-gfm][^c].

My [cmark-gfm][][^c].

My [cmark-gfm] [^c].

My [cmark-gfm[^c]].

[cmark-gfm]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702

[^c]: cmark-gfm is under the MIT license, so incorporating parts of its
    test suite into pulldown-cmark should be fine.


My [otherlink[^c]].

[otherlink[^c]]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702
"##;
    let expected = r##"<p>My [cmark-gfm]<sup class="footnote-reference"><a href="#c">1</a></sup>.</p>
<p>My <a href="https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702">cmark-gfm</a><sup class="footnote-reference"><a href="#c">1</a></sup>.</p>
<p>My <a href="https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702">cmark-gfm</a><sup class="footnote-reference"><a href="#c">1</a></sup>.</p>
<p>My <a href="https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702">cmark-gfm</a> <sup class="footnote-reference"><a href="#c">1</a></sup>.</p>
<p>My [cmark-gfm<sup class="footnote-reference"><a href="#c">1</a></sup>].</p>
<div class="footnote-definition" id="c"><sup class="footnote-definition-label">1</sup>
<p>cmark-gfm is under the MIT license, so incorporating parts of its
test suite into pulldown-cmark should be fine.</p>
</div>
<p>My [otherlink<sup class="footnote-reference"><a href="#c">1</a></sup>].</p>
<p>[otherlink<sup class="footnote-reference"><a href="#c">1</a></sup>]: https://github.com/github/cmark-gfm/blob/1e230827a584ebc9938c3eadc5059c55ef3c9abf/test/extensions.txt#L702</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_16() {
    let original = r##"[^1]: footnote definition text

<!-- -->

    // indented code block
    fn main() {
        println!("hello world!");
    }
"##;
    let expected = r##"<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
<p>footnote definition text</p>
</div>
<!-- -->
<pre><code>// indented code block
fn main() {
    println!("hello world!");
}
</code></pre>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_17() {
    let original = r##"[^1]: footnote definition text
[^1]\: this is a reference, rather than a definition
"##;
    let expected = r##"<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
<p>footnote definition text
<sup class="footnote-reference"><a href="#1">1</a></sup>: this is a reference, rather than a definition</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_18() {
    let original = r##"[^1]:

    | column1 | column2 |
    |---------|---------|
    | row1a   | row1b   |
    | row2a   | row2b   |
"##;
    let expected = r##"<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><table>
<thead>
<tr><th>column1</th><th>column2</th></tr>
</thead>
<tbody>
<tr><td>row1a</td><td>row1b</td></tr>
<tr><td>row2a</td><td>row2b</td></tr>
</tbody>
</table>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_19() {
    let original = r##"* First
  [^1]: test
* Second [^1] test


> first
> [^2]: test
> Second [^2] test


   First   | Second
-----------|----------
first      | second
[^3]: test | test [^3]


|    First   | Second    |
|------------|-----------|
| first      | second    |
| [^4]: test | test [^4] |

> [^5]: * test [^5]
"##;
    let expected = r##"<ul>
<li>First
<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
<p>test</p>
</div>
</li>
<li>Second <sup class="footnote-reference"><a href="#1">1</a></sup> test</li>
</ul>
<blockquote>
<p>first</p>
<div class="footnote-definition" id="2"><sup class="footnote-definition-label">2</sup>
<p>test
Second <sup class="footnote-reference"><a href="#2">2</a></sup> test</p>
</div>
</blockquote>
<table><thead><tr><th>First</th><th>Second</th></tr></thead><tbody>
<tr><td>first</td><td>second</td></tr>
</tbody></table>
<div class="footnote-definition" id="3"><sup class="footnote-definition-label">3</sup>
<p>test | test <sup class="footnote-reference"><a href="#3">3</a></sup></p>
</div>
<table><thead><tr><th>First</th><th>Second</th></tr></thead><tbody>
<tr><td>first</td><td>second</td></tr>
<tr><td>[^4]: test</td><td>test [^4]</td></tr>
</tbody></table>
<blockquote>
<div class="footnote-definition" id="5"><sup class="footnote-definition-label">4</sup>
<ul>
<li>test <sup class="footnote-reference"><a href="#5">4</a></sup></li>
</ul>
</div>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_20() {
    let original = r##"Test [^] link

[^]: https://rust-lang.org
"##;
    let expected = r##"<p>Test <a href="https://rust-lang.org">^</a> link</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_21() {
    let original = r##"[^foo\
bar]: not a footnote definition

[baz\
quux]: https://rust-lang.org

[first
second]: https://rust-lang.org

[^third
fourth]: not a footnote definition

[baz\
quux]
[^foo\
bar]
[first
second]
[^third
fourth]
"##;
    let expected = r##"<p>[^foo<br>
bar]: not a footnote definition</p>
<p>[^third
fourth]: not a footnote definition</p>
<p><a href="https://rust-lang.org">baz<br>
quux</a>
[^foo<br>
bar]
<a href="https://rust-lang.org">first
second</a>
[^third
fourth]</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_22() {
    let original = r##"[^foo
]: https://rust-lang.org

[^foo
]
"##;
    let expected = r##"<p><a href="https://rust-lang.org">^foo
</a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn footnotes_test_23() {
    let original = r##"footnote [^baz]
footnote [^quux]

    [^quux]: x

   [^baz]: x
"##;
    let expected = r##"<p>footnote <sup class="footnote-reference"><a href="#baz">1</a></sup>
footnote [^quux]</sup></p>
<pre><code>[^quux]: x
</code></pre>
<div class="footnote-definition" id="baz"><sup class="footnote-definition-label">1</sup>
<p>x</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}
