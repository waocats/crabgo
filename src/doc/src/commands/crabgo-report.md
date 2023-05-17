# crabgo-report(1)

## NAME

crabgo-report --- Generate and display various kinds of reports

## SYNOPSIS

`crabgo report` _type_ [_options_]

### DESCRIPTION

Displays a report of the given _type_ --- currently, only `future-incompat` is supported

## OPTIONS

<dl>

<dt class="option-term" id="option-crabgo-report---id"><a class="option-anchor" href="#option-crabgo-report---id"></a><code>--id</code> <em>id</em></dt>
<dd class="option-desc">Show the report with the specified Crabgo-generated id</dd>


<dt class="option-term" id="option-crabgo-report--p"><a class="option-anchor" href="#option-crabgo-report--p"></a><code>-p</code> <em>spec</em>…</dt>
<dt class="option-term" id="option-crabgo-report---package"><a class="option-anchor" href="#option-crabgo-report---package"></a><code>--package</code> <em>spec</em>…</dt>
<dd class="option-desc">Only display a report for the specified package</dd>


</dl>

## EXAMPLES

1. Display the latest future-incompat report:

       crabgo report future-incompat

2. Display the latest future-incompat report for a specific package:

       crabgo report future-incompat --package my-dep:0.0.1

## SEE ALSO
[Future incompat report](../reference/future-incompat-report.html)

[crabgo(1)](crabgo.html)
