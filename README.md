# NAME

kraken-websockets-token - Convenience crate to get Kraken WebSockets API tokens

## Example

    use kraken_websockets_token::get_websockets_token;

    fn main() {
      const API_SECRET: &str = "<my API secret>";
      const API_KEY: &str = "<my API key>";

      match get_websockets_token(API_SECRET, API_KEY) {
        Ok(token) => println!("My token is: {}", token),
        Err(err) => println!("There was an error: {}", err),
      }
    }

# DESCRIPTION

This crate provides a convenient way to get Kraken WebSockets API tokens.

# SUPPORT

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/kraken-websockets-token/issues](https://github.com/alfiedotwtf/kraken-websockets-token/issues)

Feel free to fork the repository and submit pull requests :)

# SEE ALSO

* [Kraken WebSockets authentication documentation](https://www.kraken.com/features/api#ws-auth)

* [Kraken WebSockets API 1.1.0](https://docs.kraken.com/websockets/#authentication)

# AUTHOR

[Alfie John](https://www.alfie.wtf)

# WARRANTY

IT COMES WITHOUT WARRANTY OF ANY KIND.

# COPYRIGHT AND LICENSE

Copyright (C) 2021 by Alfie John

Contact me if you would like this licensed under something other than the GPL

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
