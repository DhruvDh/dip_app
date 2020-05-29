from glob import glob
import os
import subprocess

file_paths = glob("./static/*.md");
comrak = "comrak --gfm --github-pre-lang --smart --unsafe -e strikethrough -e table -e autolink -e tasklist -e tagfilter -e superscript -e footnotes -e description-lists"


for path in file_paths:
    out_file = path.replace(".md", ".vue")
    out_file = out_file.replace("./static", "./js/components/mdPages")
    
    html = subprocess.check_output(f"{comrak} {path}").decode('utf-8')

    template = f"""
<template>
    <div class="markdown-body">
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
</style>
    """

    component = f"""
{template}
{script}
{style}
    """

    with open(out_file, 'w', encoding='utf-8') as out:
        out.write(component)
        print(f"wrote {out_file}")