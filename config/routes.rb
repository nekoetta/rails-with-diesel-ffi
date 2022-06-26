Rails.application.routes.draw do
  # Define your application routes per the DSL in https://guides.rubyonrails.org/routing.html

  # Defines the root path route ("/")
  root "csv#index"
  get 'ruby', to: 'csv#ruby'
  get 'rust', to: 'csv#rust'
end
