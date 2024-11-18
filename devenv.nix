{ pkgs, lib, config, inputs, ... }:
let
  dbName = "uno";
  dbHost = "127.0.0.1";
in 
{
  config = {
    env = {
      DATABASE_URL="mysql://admin:1234@${dbHost}:3306/${dbName}";
    };

    services.linusfri.mysql = {
      enable = true;

      dbName = dbName;
    };

    packages = with pkgs; [
      git
      diesel-cli
      cargo-watch
    ];

    languages.rust.enable = true;

    # processes.cargo-watch.exec = "cargo watch -x run";
    processes.app.exec = "./result/bin/uno_api";
  };
}
