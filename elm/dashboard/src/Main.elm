module Main exposing (..)
-- Press a button to send a GET request for random cat GIFs.
--
-- Read how it works:
--   https://guide.elm-lang.org/effects/json.html
--

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as JD exposing (Decoder, field, string)



-- MAIN


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }



-- MODEL


type Model
  = Failure
  | Loading
  | Success (List Product)


type alias Product = { thumbnail : String }


init : () -> (Model, Cmd Msg)
init _ =
  (Loading, getProducts)



-- UPDATE


type Msg
  = MorePlease
  | GotProducts (Result Http.Error (List Product))


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    MorePlease ->
      (Loading, getProducts)

    GotProducts result ->
      case result of
        Ok products ->
          (Success products, Cmd.none)

        Err _ ->
          (Failure, Cmd.none)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ h2 [] [ text "Random Cats" ]
    , viewProducts model
    , button [ onClick MorePlease ] [ input [ value "Type a Query!"] [] ]
    ]


viewProducts : Model -> Html Msg
viewProducts model =
  case model of
    Failure ->
      div []
        [ text "Failed to search. "
        , button [ onClick MorePlease ] [ text "Try Again!" ]
        ]

    Loading ->
      text "Loading..."

    Success [] ->
      text "Nothing..."

    Success products ->
      div [] (List.map viewThumb products)


viewThumb : Product -> Html Msg
viewThumb product = img [ src product.thumbnail ] []

-- HTTP


getProducts : Cmd Msg
getProducts =
  Http.get
    { url = "https://api.mercadolibre.com/sites/MLB/search?q=rk3399#options"
    , expect = Http.expectJson GotProducts productsDecoder
    }


productsDecoder : Decoder (List Product)
productsDecoder =
  field "results" (JD.list productDecoder)

productDecoder : Decoder Product
productDecoder = JD.map Product (field "thumbnail" string)
