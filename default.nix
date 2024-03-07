{ sources ? import ./nix/sources.nix # managed by https://github.com/nmattia/niv
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs {
  overlays = [
    # gomod2nix must be imported as a nixpkgs overlay
    (import (sources.gomod2nix+"/overlay.nix"))
  ];
}
, meta ? pkgs.lib.importJSON ./nix/meta.json
, dockerName ? "docker.stackable.tech/sandbox/${meta.operator.name}"
, dockerTag ? null
}:
rec {
  cargo = import ./Cargo.nix {
    inherit nixpkgs pkgs; release = false;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      prost-build = attrs: {
        buildInputs = [ pkgs.protobuf ];
      };
      tonic-reflection = attrs: {
        buildInputs = [ pkgs.rustfmt ];
      };
      stackable-secret-operator = attrs: {
        buildInputs = [ pkgs.protobuf pkgs.rustfmt ];
      };
      krb5-sys = attrs: {
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ (pkgs.enableDebugging pkgs.krb5) ];
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include -I${pkgs.clang.cc.lib}/lib/clang/${pkgs.lib.getVersion pkgs.clang.cc}/include";
      };
      stackable-cockpit-web = attrs: {
        nativeBuildInputs = [ pkgs.nodePackages.yarn pkgs.nodejs ];
        preConfigure =
          ''
            [[ ! -e node_modules ]] || rm -r node_modules
            ln -s ${web.nodeModules} node_modules
          '';
      };
      helm-sys = attrs: {
        GO_HELM_WRAPPER = goHelmWrapper + "/bin";
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include -I${pkgs.clang.cc.lib}/lib/clang/${pkgs.lib.versions.major (pkgs.lib.getVersion pkgs.clang.cc)}/include";
      };
    };
  };
  build = cargo.workspaceMembers.stackable-cockpitd.build.override {
    features = [ "ui" ];
  };
  entrypoint = build+"/bin/stackable-cockpitd";
  # crds = pkgs.runCommand "${meta.operator.name}-crds.yaml" {}
  # ''
  #   ${entrypoint} crd > $out
  # '';

  dockerImage = pkgs.dockerTools.streamLayeredImage {
    name = dockerName;
    tag = dockerTag;
    contents = [
      # Common debugging tools
      pkgs.bashInteractive pkgs.coreutils pkgs.util-linuxMinimal
      # Kerberos 5 must be installed globally to load plugins correctly
      pkgs.krb5
      # Make the whole cargo workspace available on $PATH
      build
    ];
    config = {
      Env =
        let
          fileRefVars = {
            PRODUCT_CONFIG = deploy/config-spec/properties.yaml;
          };
        in pkgs.lib.concatLists (pkgs.lib.mapAttrsToList (env: path: pkgs.lib.optional (pkgs.lib.pathExists path) "${env}=${path}") fileRefVars);
      Entrypoint = [ entrypoint ];
      Cmd = [];
    };
  };
  docker = pkgs.linkFarm "stackable-cockpit-docker" [
    {
      name = "load-image";
      path = dockerImage;
    }
    {
      name = "ref";
      path = pkgs.writeText "${dockerImage.name}-image-tag" "${dockerImage.imageName}:${dockerImage.imageTag}";
    }
    {
      name = "image-repo";
      path = pkgs.writeText "${dockerImage.name}-repo" dockerImage.imageName;
    }
    {
      name = "image-tag";
      path = pkgs.writeText "${dockerImage.name}-tag" dockerImage.imageTag;
    }
    # {
    #   name = "crds.yaml";
    #   path = crds;
    # }
  ];

  # need to use vendored crate2nix because of https://github.com/kolloch/crate2nix/issues/264
  crate2nix = import sources.crate2nix {};
  js2nix = pkgs.callPackage sources.js2nix { nodejs = pkgs.nodejs-18_x; };
  gomod2nix = pkgs.callPackage sources.gomod2nix {};
  tilt = pkgs.tilt;

  web = js2nix.buildEnv {
    # js2nix doesn't import peer dependencies, so we use overlays to patch them in explicitly
    # https://github.com/canva-public/js2nix/blob/d37912f6cc824e7f41bea7a481af1739ca195c8f/docs/usage.md#overriding
    package-json = ./web/package.json;
    yarn-lock = ./yarn.lock;
    overlays = [
      (self: super: {
        # TODO: remove once this https://github.com/canva-public/js2nix/issues/20 is resolved
        buildNodeModule = pkgs.lib.makeOverridable
          (args: (super.buildNodeModule args).override { doCheck = false; });
      })
    ];
  };

  goHelmWrapper = pkgs.buildGoApplication {
    pname = "go-helm-wrapper";
    version = "0.0";
    src = pkgs.runCommand "go-helm-wrapper-src" {}
      ''
        mkdir $out
        cp ${./go.mod} $out/go.mod
        cp ${./go.sum} $out/go.sum
        cp -r ${./rust/helm-sys/go-helm-wrapper} $out/go-helm-wrapper
      '';
    pwd = ./rust/helm-sys/go-helm-wrapper;
    modules = ./gomod2nix.toml;
    ldflags = "-buildmode c-archive";
    allowGoReference = true;
    postBuild =
      ''
        for pkg in $(getGoDirs ""); do
          buildFlags="-buildmode c-archive -o $GOPATH/bin/libgo-helm-wrapper.a" buildGoDir build "$pkg"
        done
      '';
  };

  regenerateNixLockfiles = pkgs.writeScriptBin "regenerate-nix-lockfiles"
    ''
      #!/usr/bin/env bash
      set -euo pipefail
      echo Running crate2nix
      ${crate2nix}/bin/crate2nix generate
      echo Running gomod2nix
      ${gomod2nix}/bin/gomod2nix
    '';
}
