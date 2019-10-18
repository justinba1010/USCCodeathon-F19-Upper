#!/usr/bin/perl
# Harrison Howell
# Create test cases
# Upper Division

use 5.026_001;
use strict;
use warnings;
use utf8;

my $cases = 15;

my @desc_chars = map(chr, ord("\N{U+3041}")..ord("\N{U+3093}"));
push(@desc_chars, (map(chr, ord("\N{U+30A1}")..ord("\N{U+30F3}"))));
push(@desc_chars, (map(chr, ord("\N{U+3190}")..ord("\N{U+319F}"))));
push(@desc_chars, (map(chr, ord("\N{U+FF01}")..ord("\N{U+FF6B}"))));
push(@desc_chars, (map(chr, ord("\N{U+FF70}")..ord("\N{U+FFBE}"))));

`rm -rf ./input ./output`;
`mkdir ./input ./output`;
# It's nice to know if you're on the right track!
`cp ./example-input input/input00.txt`;
`perl ./solutions/solution.pl <./input/input00.txt >./output/output00.txt`;
die("$!") if ($? >> 8 != 0);

foreach (1..$cases) {
  my $file = "0$_.txt";
  open FILE_OUT, '>:utf8', "input/input$file" or die("$!\n");
  print "writing $file...\n";

  srand($_);

  foreach my $entry (1..$_*2) {
    print FILE_OUT "\$$entry.00 Entry $entry for $file\n";

    my $desc;
    $desc .= $desc_chars[rand @desc_chars] for 1..2**(int(rand($_)));
    for (my $i = 0; $i < length($desc); $i += 40) {
      print FILE_OUT substr($desc, $i, 40), "\n";
    }
  }

  `perl ./solutions/solution.pl <./input/input$file >./output/output$file`;
  die("$!") if ($? >> 8 != 0);
  close FILE_OUT;
}

srand(++$cases);
open FILE_OUT, '>:utf8', "input/input0$cases.txt" or die("$!\n");
print FILE_OUT "\$1.00 Entry 1 for input0$cases\n";
my $desc = '';
$desc .= $desc_chars[rand @desc_chars] for 1..2**16;
for (my $i = 0; $i < length($desc); $i += 40) {
  print FILE_OUT substr($desc, $i, 40), "\n";
}
`perl ./solutions/solution.pl <./input/input0$cases.txt >./output/output0$cases.txt`;
print "Done!\n";
