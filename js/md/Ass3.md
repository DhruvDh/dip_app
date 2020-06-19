# Assignment 3 - Part B

- [Assignment 3 - Part B](#assignment-3---part-b)
  - [Introduction](#introduction)
  - [Applying Image Filters](#applying-image-filters)
  - [What to do](#what-to-do)
  - [Submission Template](#submission-template)

## Introduction

This assignment is about Image Filtering, the goal of which is to get familiar with how to do point transforms. Refer to the notes on quiz for Image Filtering if needed.

## Applying Image Filters

The animations from the notes on the quiz showed how to perform convolutions or image filtering on a single color channel, for example this -

![convolutions](https://raw.githubusercontent.com/DhruvDh/dip_image_viewer/master/docs/Convolution.png)

And so on -

![convolutions-animation](https://dhruvdh.github.io/dip_image_viewer/Convolution_animation.gif)

The "image" matrix in both of these illustrations show only a single color channel, say Red, and when we perform convolutions on color images we have to repeat the process for each color channel.

## What to do

The goal is to write a program that can produce an output image file after applying the image filter to all color channels from the input file. 

I would highly recommend stacking the image in 2D ararys for each color channel separately, so that carrying out the convolution is easier. Once you've performed filtering on all three color channels, you can put them together and write them out to the output file.

As an example, if I had this image -
```
  RGB
  4
  4
  0 0 1
  0 2 0
  0 0 3
  0 4 0
  0 5 0
  6 0 0
  0 7 0
  8 0 0
  0 0 9
  0 10 0
  0 0 11
  0 12 0
  0 13 0
  14 0 0
  0 15 0
  16 0 0
```

And I were to stack just the blue color channel in a 2D array, I would end up with -
```python
[ # blue
  [1, 0,  3, 0],
  [0, 0,  0, 0],
  [9, 0, 11, 0],
  [0, 0,  0, 0]
]
```

Refer to the quiz on Image Filtering which lays out the actual process of calculating in detail. You can copy paste text from the third column to the text box in the top of this page to see the output from the applying the kernel.

| Kernal Name           | Kernel Value                                                                                                | Kernel as an unscaled 2D Python Array                                                                                                                  |
| --------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Identity              | ![Kernel](https://wikimedia.org/api/rest_v1/media/math/render/svg/5bf6623ca763ba780b471a565eb1b06cd14b445c) | `[ [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0] ]`                                                                                                |
| Edge detection        | ![Kernel](https://wikimedia.org/api/rest_v1/media/math/render/svg/f800ad5f76b6c26c729ff0c1fef44284d7cade7a) | `[ [-1.0, -1.0, -1.0], [-1.0, 8.0, -1.0], [-1.0, -1.0, -1.0] ]`                                                                                        |
| Sharpen               | ![Kernel](https://wikimedia.org/api/rest_v1/media/math/render/svg/beb8b9a493e8b9cf5deccd61bd845a59ea2e62cc) | `[ [0.0, -1.0, 0.0], [-1.0, 5.0, -1.0], [0.0, -1.0, 0.0] ]`                                                                                            |
| Box Blur              | ![Kernel](https://wikimedia.org/api/rest_v1/media/math/render/svg/f1e6d5ec15af752f471372b96a1be4a83e02873e) | `[ [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0] ]`                                                                                                |
| Gaussian Blur (5x5)   | ![Kernel](https://wikimedia.org/api/rest_v1/media/math/render/svg/f91401a3e97428f14862afa1c781c55f4157580b) | `[ [1.0, 4.0, 6.0, 4.0, 1.0], [4.0, 16.0, 24.0, 16.0, 4.0], [6.0, 24.0, 36.0, 24.0, 6.0], [4.0, 16.0, 24.0, 16.0, 4.0], [1.0, 4.0, 6.0, 4.0, 1.0] ]`   |
| Unsharp Masking (5x5) | ![Kernel](https://wikimedia.org/api/rest_v1/media/math/render/svg/259aa5d59203cb5fd7a04263ae9c09301c70dc97) | `[ [1.0, 4.0, 6.0, 4.0, 1.0], [4.0, 16.0, 24.0, 16.0, 4.0], [6.0, 24.0, -476.0, 24.0, 6.0], [4.0, 16.0, 24.0, 16.0, 4.0], [1.0, 4.0, 6.0, 4.0, 1.0] ]` |

## Submission Template

You are to produce 3 output file (one for each kernel - Identity, Edge detection, and Gaussian Blur, from the above table) for each input file in the zip, take screenshots of the same using the [Viewer](https://dhruvdh.github.io/dip_app/#/viewer), and submit your code, along with the output files and the screenshots.