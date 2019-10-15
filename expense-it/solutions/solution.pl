#!/usr/bin/perl
# Harrison Howell
# Reverse-complement for Codeathon 2019 Fall
# Upper Division

use 5.026_001;
use strict;
use warnings;
use utf8;
binmode(STDIN, ':utf8');
binmode(STDOUT, ':utf8');

# Seperate input by '$' instead of '\n'
local $/ = '$';
while (my $report = <STDIN>) {
  chomp($report);

  my ($price, $desc) = split /\n/, $report, 2;
  next if (!$price);

  {
    local $/ = "\n";
    print '$', $price, "\n";

    $desc = reverse($desc);
    # The unicode ranges cover all Kanakana, Hiragana, and half width kana
    # it's just a matter of arranging them
    $desc =~ tr{\N{U+3041}-\N{U+3093}\N{U+30A1}-\N{U+30F3}\N{U+FF67}-\N{U+FF6B}\n}
               {\N{U+30A1}-\N{U+30F3}\N{U+3041}-\N{U+3093}\N{U+FF71}-\N{U+FF75}}d;

    # For simplicity '\n' is removed with the 'd' modifier above so the string
    # can be formatted correctly
    for (my $i = 0; $i < length($desc); $i += 40) {
      print substr($desc, $i, 40), "\n";
    }
  }
}
