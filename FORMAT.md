# pixie format

pixie uses `.px` files to describe small pixel art images.

## settings

Settings start with `$`.

```text
$scale = 16
$resolution = 128 128
$auto = [resolution]
````

### scale

Scales the source image by an integer factor.

```text
$scale = 16
```

### resolution

Sets the output resolution.

```text
$resolution = 128 128
```

### auto

Enables automatic processing features.

Currently available:

```text
$auto = [resolution, scale]
```

`resolution` reconstructs the source image at the target resolution.
`scale` scales the source image using the value from $scale.

## palette

Palette entries map a character to an RGB color.

```text
# = #FF0000
% = #000000
```

Symbols must contain exactly one character. Lowercase letters `a-z` are reserved.

## pixel grid

The first line that is not a setting or palette entry starts the pixel grid.

```text
%%%%%%%%
%%####%%
%##%%##%
%#%%%%#%
%%%%%%%%
```

Every row must have the same width, and every symbol must exist in the palette.

## comments

Comments start with `;`.
```text
; This is a comment
```

## complete example

```text
$scale = 1028
$resolution = 1028 1028
$auto = [scale, resolution]

# = #FFFFFF
@ = #000000
* = #D395E5

################
################
##@@########@@##
##@@########@@##
##@@########@@##
##@@########@@##
################
###*########*###
################
###@@######@@###
###@@######@@###
###@@@@@@@@@@###
################
################
################
################
```