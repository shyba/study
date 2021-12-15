module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as JD exposing (Decoder, field, string, float)



-- MAIN


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }



-- MODEL


type alias Model
  = { products: (List Product), error: Bool, query: String }


type alias Product = { thumbnail : String, permalink: String, title: String, price: Float }


init : () -> (Model, Cmd Msg)
init _ =
  ((Model [] False ""), getProducts "")



-- UPDATE


type Msg
  = MorePlease
  | UpdateQuery String
  | GotProducts (Result Http.Error (List Product))


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    MorePlease ->
      ((Model [] False ""), getProducts model.query)

    UpdateQuery query ->
        ({ model | query = query }, Cmd.none)

    GotProducts result ->
      case result of
        Ok products ->
          ({ model | products = products }, Cmd.none)

        Err _ ->
          ({ model | error = True } , Cmd.none)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ h2 [] [ text "Busca Mercado Livre" ]
    , viewProducts model
    , Html.form [ onSubmit MorePlease ] [ input [ onInput UpdateQuery ] [text "Search!"] ]
    ]


viewProducts : Model -> Html Msg
viewProducts model =
  case model.error of
    True ->
      div []
        [ text "Failed to search. "
        , button [ onClick MorePlease ] [ text "Try Again!" ]
        ]


    False ->
      div [] (List.map viewThumb model.products)


viewThumb : Product -> Html Msg
viewThumb product = div [ style "max-width" "20%", style "display" "inline-block"] [
    p [ style "max-width" "20%"] [text product.title]
    ,a [href product.permalink] [
        img [ src product.thumbnail, style "max-width" "100px" ] []
    ]
    ]

-- HTTP


getProducts : String -> Cmd Msg
getProducts query =
    case String.isEmpty query of
         True ->
            Http.get
              { url = "https://api.mercadolibre.com/sites/MLB/search?q=rk3399"
              , expect = Http.expectJson GotProducts productsDecoder
              }
         False ->
            Http.get
              { url = "https://api.mercadolibre.com/sites/MLB/search?q=" ++ query
              , expect = Http.expectJson GotProducts productsDecoder
              }


productsDecoder : Decoder (List Product)
productsDecoder =
  field "results" (JD.list productDecoder)

productDecoder : Decoder Product
productDecoder = JD.map4 Product (field "thumbnail" string) (field "permalink" string) (field "title" string) (field "price" float)
