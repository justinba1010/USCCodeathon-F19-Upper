#!/usr/bin/perl

use 5.026_001;
use strict;
use utf8;
binmode(STDIN, ':utf8');
binmode(STDOUT, ':utf8');

local $/ = '$';
while (my $record = <STDIN>) {
  chomp($record);

  my ($item, $info) = split /\n/, $record, 2;
  next if (!$item);

  {
    local $/ = "\n";
    print '$', $item, "\n";

    $info = reverse($info);
    $info =~ tr{\N{U+3042}-\N{U+3093}\N{U+30A2}-\N{U+30F3}\N{U+FF67}-\N{U+FF6B}\n}
               {\N{U+30A2}-\N{U+30F3}\N{U+3042}-\N{U+3093}\N{U+FF71}-\N{U+FF75}}d;

    for (my $i = 0; $i < length($info); $i += 40) {
      print substr($info, $i, 40), "\n";
    }
  }
}
