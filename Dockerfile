FROM ruby:latest

WORKDIR /usr/src

COPY Gemfile Gemfile.lock ./
RUN apt-get update; \
    gem install bundler; \
    bundle install --path vendor/bundle;

RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash ; \
    apt-get install -y nodejs; \
    npm install --global yarn;

RUN rm -rf /var/lib/apt/lists/*;

EXPOSE 3000

COPY ./docker-entrypoint.sh /tmp
ENTRYPOINT ["/tmp/docker-entrypoint.sh"]
CMD ["bin/rails", "server", "-b", "0.0.0.0"]
