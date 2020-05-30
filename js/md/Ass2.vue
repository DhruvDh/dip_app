

<template>
    <div class="markdown-body container" style="margin-top: 1rem">
<h2>Table of Contents</h2>
<ul>
<li><a href="#table-of-contents">Table of Contents</a></li>
<li><a href="#introduction">Introduction</a>
<ul>
<li><a href="#overview">Overview</a></li>
<li><a href="#more-details">More details</a></li>
<li><a href="#output-format">Output Format</a></li>
<li><a href="#submission-format">Submission Format</a></li>
</ul>
</li>
<li><a href="#part-a">Part A</a>
<ul>
<li><a href="#an-example">An example</a></li>
</ul>
</li>
<li><a href="#part-b">Part B</a></li>
<li><a href="#part-c">Part C</a></li>
</ul>
<h2>Introduction</h2>
<p>As I eluded to in the last lecture, assignment 2 is going to be about demosaicing. Demosacing algorithms are what are used to produce “normal” looking color images from camera sensors with color filter arrays installed on them. It will consist of 3 parts, each with it’s own goal</p>
<h3>Overview</h3>
<ul>
<li>Part A: Goal is to get comfortable with the image format used for the course, along with using this website as a “viewer”. You will stack the output produced by the camera sensor into a grid, and assign each value a color by referencing the bayer color filter pattern and the location of the intensity value in the grid.</li>
<li>Part B: Goal is to perform simple demosaicing and combine 4 neighboring intensity values of their respective color into one RGB color; and do the same for all pixels.</li>
<li>Part C: Goal is to perform a slightly more complicated demosaicing algorithm, that involves a “sliding window”. You will notice how depending on the choice of demosaicing algorithm the resolution of the image we produce differs.</li>
</ul>
<blockquote>
<p><strong>Note that you are not to use any library outside of what is available by default in the programming language you choose.</strong></p>
</blockquote>
<h3>More details</h3>
<ul>
<li>Just like before there are 3 text files for you, you won’t have to change any code or redo any work for each except for change the width and the height numbers. You just have to run your code for each text file.</li>
<li>You will be given a list of intensity values, just like before; but this time instead of representing raw data produced by each photodiode, each intensity value that is scaled up to <code>0-255</code> to match 8-bit color.</li>
<li>For the previous assignment, the we treated each value, equally, without think about color.</li>
<li>This time around, we will assume that each photodiode has a color filter installed in front of it. You can figure out which color filter it is by consulting the bayer color filter array pattern. There are multiple GIFs on this page that also serve as a reference for the bayer pattern.</li>
<li>The <code>_easy</code> and the <code>_medium</code> images are meant to by <code>256</code> by <code>256</code> pixels.</li>
<li>According to the Bayer pattern, <code>4</code> photodiodes combine to produce a single color pixel. Meaning you are supplied with <code>256 * 256 * 4</code> values.</li>
</ul>
<h3>Output Format</h3>
<ul>
<li>
<p>As an “output” for each Part of the assignment, you will be creating a text file using the program you write. Each part is going to ask of you to create an image, and each image will be in this following format ––</p>
<ul>
<li>The first line in the file will be <code>RGB</code> to represent that the image uses the RGB colorspace. <code>RGB</code> will be followed by a newline <code>\n</code>, so - <code>RGB\n</code>.</li>
<li>The second line will be a number representing the height of the image in pixels, so <code>256\n</code> for an image consisting of 256 pixels.</li>
<li>The third line in the image would be the width of the image in pixels, so <code>512\n</code> for a 512 pixel wide image.</li>
<li>From the fourth line onwards you will have the RGB color for each pixel, so <code>144 72 44\n</code> for the color <code>rgb(144, 72, 44)</code>. You will have as many lines as pixels in your image.</li>
</ul>
</li>
</ul>
<p>You can use the “Viewer” section of this website to look at these images.</p>
<h3>Submission Format</h3>
<p>You will submit as a zip ––</p>
<ul>
<li>Your text file that was provided as input.</li>
<li>Source code of your program that implements the solution.</li>
<li>Output files produced by your program.</li>
<li>Screenshot of these output files as viewed in the viewer.</li>
</ul>
<h2>Part A</h2>
<blockquote>
<p>Part A is worth 20 points.</p>
</blockquote>
<p>-The first task is simply to convert the list of intensity values given to the above image format, take a screenshot of the images using the supplied binary, and attach both the image text files and the screenshots and submit.</p>
<ul>
<li>
<p>You are to also describe, briefly, what you see in the image and why you think it looks the way it does.</p>
</li>
<li>
<p>Last time around you were given a list of voltage values, which you stacked (either to make a CSV file, ASCII art, or something else), like so -
<img src="img/parta_a.gif" alt="Stacking" /></p>
</li>
<li>
<p>Those intensity values represented the output of a grayscale camera sensor, without color. This time around, the images have a bayer color filter array in front of them, like so -
<img src="img/parta_.gif" alt="colored" /></p>
</li>
<li>
<p>Meaning each intensity value now has an associated color.</p>
</li>
<li>
<p>Your task is to first stack the intensity values in a grid, so that you can use the bayer pattern to figure out which value pertains to which color.</p>
</li>
<li>
<p>Once you’ve done that, you’ll turn each value into an RGB color. For instance, the <code>83</code> when meant for a red color turns into <code>83 0 0</code>; for a green color turns into <code>0 83 0</code> and for a blue color turns into <code>0 0 83</code>.</p>
</li>
<li>
<p>Once you’ve figured out the RGB value for each intensity value, write it out to an output file in the format mentioned above.</p>
</li>
<li>
<p>In these animations, I’ve shown a camera sensor with <code>16</code> photodiodes giving you <code>16</code> intensity values, and I’ve stacked them into a <code>4</code> by <code>4</code> grid so that I can apply the bayer pattern to tell me which intensity value belongs to what color.</p>
</li>
<li>
<p>In the <code>_easy</code> and the <code>_medium</code> text files you are given, there are <code>‭262,144‬</code> intensity values, which will be stacked in a 512 by 512 grid ––</p>
<ul>
<li>We know that the <code>_easy</code> image is <code>256</code> pixels by <code>256</code> pixels.</li>
<li>We know from the Color acquisition lecture that the color from each pixel is derived from <code>4</code> photodiodes.</li>
<li>We know that these photodiodes that produce a single colored pixel are arranged in a <code>2</code> by <code>2</code> grid.</li>
<li>Meaning that if the image is <code>256</code> by <code>256</code> pixels, then the camera sensor would have a grid of <code>256 * 2</code> by <code>256 * 2</code> photodiodes giving us a <code>512</code> by <code>512</code> grid.</li>
</ul>
</li>
</ul>
<h3>An example</h3>
<ul>
<li>Assume a <code>2</code> by <code>2</code> pixel image.</li>
<li>The sensor would have <code>4</code> by <code>4</code> photodiodes, producing <code>4 * 4</code> intensity values.</li>
<li>Let’s say those values are - <code>[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]</code></li>
<li>You stack them in a grid to produce ––
<pre><code>[
  [1, 2, 3, 4],
  [5, 6, 7, 8],
  [9, 10, 11, 12],
  [13, 14, 15, 16]
]
</code></pre>
</li>
<li>Once you’ve done that, you can use the bayer pattern to realise that <code>1</code> is the intensity for  the color blue, <code>2</code> is for green, <code>3</code> is for blue again, and <code>4</code> is for green again. For the next row, the patten tells us that <code>5</code> belongs to green, <code>6</code> belongs to red, so on and so forth.</li>
<li>This would produce the following file, according to the format described above––
<pre><code>RGB
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
</code></pre>
</li>
<li>We used <code>4</code> pixels as the height for the image, and <code>4</code> pixels as the width, because we’re not combining the individual intensities into color yet, so a <code>4</code> by <code>4</code> photodiode grid produces a <code>4</code> by <code>4</code> image. When we do part B, we’ll actually combine the intensities, and we’ll produce a <code>2</code> by <code>2</code> image instead.</li>
</ul>
<blockquote>
<p>Do make sure the resolution you enter in the file properly corresponds to the number of pixels in the output image</p>
</blockquote>
<h2>Part B</h2>
<blockquote>
<p>Part B is worth 60 points</p>
</blockquote>
<ul>
<li>The second part is where you will actually perform the demosaicing, basically, this ––</li>
</ul>
<img src="img/partb_.gif" height="25%" width="25%">
<ul>
<li>You will be “combining” the intensity for 4 photodiodes into 1 by doing this ––</li>
</ul>
<img src="img/partb_b.gif" height="35%" width="35%">
<ul>
<li>Again, take a screenshot of the image using the Viewer, and attach both the image file and the screenshot and submit.</li>
</ul>
<blockquote>
<p>Do make sure the resolution you enter in the file properly corresponds to the number of pixels in the output image</p>
</blockquote>
<h2>Part C</h2>
<blockquote>
<p>Part C is worth 40 points</p>
</blockquote>
<ul>
<li>This time around you will be doing this instead ––</li>
</ul>
<img src="img/partb_c.gif" height="25%" width="25%">
<ul>
<li>
<p>This is a little tricky, and you’ll have to spend some time thinking about how the colors are being combined and planning how you write the algorithm.</p>
</li>
<li>
<p>Again, take a screenshot of the image using the Viewer, and attach both the image file and the screenshot and submit.</p>
</li>
</ul>
<blockquote>
<p>Do make sure the resolution you enter in the file properly corresponds to the number of pixels in the output image</p>
</blockquote>

    </div>
</template>
    

<script>
    export default {
    mounted() {
        this.$nextTick(() => {
            hljs.initHighlighting();
        });
    },
    };
</script>
    

<style lang="scss">
    @import "@primer/css/markdown/index.scss";
    
    code {
        color: #000000 !important;
    }

    ul {
        list-style-type: disc !important;
    }
</style>
    
    