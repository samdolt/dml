pub const HTML_HEADER: &'static str = "
<!doctype html>
<html lang=\"en\">
<head>
    <meta \
                                       charset=\"utf-8\">
    <script \
                                       type=\"text/x-mathjax-config\">
MathJax.Hub.Config({
  \
                                       tex2jax: {inlineMath: [['$','$'], ]},
  TeX: { \
                                       equationNumbers: { autoNumber: \"all\" }},
});
</script>
\
                                       <script type=\"text/javascript\"
  \
                                       src=\"https://cdn.mathjax.org/mathjax/latest/MathJax.\
                                       js?config=TeX-AMS-MML_HTMLorMML\">
</script>
</head>
\
                                       <body>
";

pub const HTML_FOOTER: &'static str = "
</body>
</html>
";
