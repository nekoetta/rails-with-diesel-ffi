require 'ffi'
require 'csv'

class CsvController < ApplicationController
  def index; end

  def ruby
    start_time = Time.now

    ruby_csv = CSV.generate do |csv|
      Something.preload(:user).find_each(batch_size: 10_000) do |something|
        csv << [something.user.name, something.sentence]
      end
    end

    puts '------------------------------------------------------------'
    puts '処理にかかった時間'
    puts Time.now - start_time
    puts '------------------------------------------------------------'

    respond_to do |format|
      format.csv { send_data ruby_csv }
    end
  end

  # get string data in csv format from rust program
  module CSVtaker
    extend FFI::Library

    ffi_lib "#{Rails.root}/ffi/diesel_ffi/target/release/libdiesel_ffi.so"
    attach_function :take_csv, [], :string
  end

  def rust
    start_time = Time.now

    rust_csv = CSVtaker.take_csv

    puts '------------------------------------------------------------'
    puts '処理にかかった時間'
    puts Time.now - start_time
    puts '------------------------------------------------------------'


    respond_to do |format|
      format.csv { send_data rust_csv }
    end
  end
end
