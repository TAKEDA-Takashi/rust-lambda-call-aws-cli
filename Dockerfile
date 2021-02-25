FROM public.ecr.aws/lambda/provided:al2

RUN yum install -y unzip

RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" \
    && unzip awscliv2.zip \
    && ./aws/install

# 実行ファイルを起動するようにするため、ファイル名を "bootstrap" に変更する
COPY ./target/release/rust-lambda-call-aws-cli ${LAMBDA_RUNTIME_DIR}/bootstrap

# カスタムランタイム同様ハンドラ名は利用しないため、適当な文字列を指定する。
CMD ["lambda-handler"]
