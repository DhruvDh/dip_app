from glob import glob
import os
import subprocess

file_paths = glob(".\js\md\*.md");
comrak = "comrak --gfm --github-pre-lang --smart --unsafe -e strikethrough -e table -e autolink -e tasklist -e tagfilter -e superscript -e footnotes -e description-lists"


for path in file_paths:
    print(f"read:\t{path}")
    out_file = path.replace(".md", ".vue")
    html = subprocess.check_output(f"{comrak} {path}").decode('utf-8')

    template = f"""
<template>
    <div class="markdown-body container" style="margin-top: 1rem">
{html}
    </div>
</template>
    """

    script = """
<script>
    export default {
    mounted() {
        this.$nextTick(() => {
            hljs.initHighlighting();
        });
    },
    };
</script>
    """
    
    style = """
<style lang="scss">
    @import "@primer/css/markdown/index.scss";
    
    code {
        color: #000000 !important;
    }

    ul {
        list-style-type: disc !important;
    }
</style>
    """

    component = f"""
{template}
{script}
{style}
    """

    with open(out_file, 'w', encoding='utf-8') as out:
        out.write(component)
        print(f"wrote:\t{out_file}")