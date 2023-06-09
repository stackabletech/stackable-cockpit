/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */


/** OneOf type helpers */
type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };
type XOR<T, U> = (T | U) extends object ? (Without<T, U> & U) | (Without<U, T> & T) : T | U;
type OneOf<T extends any[]> = T extends [infer Only] ? Only : T extends [infer A, infer B, ...infer Rest] ? OneOf<[XOR<A, B>, ...Rest]> : never;

export interface paths {
  "/demos": {
    /**
     * Retrieves all demos. 
     * @description Retrieves all demos.
     */
    get: operations["get_demos"];
  };
  "/demos/{name}": {
    /**
     * Retrieves one demo identified by `name`. 
     * @description Retrieves one demo identified by `name`.
     */
    get: operations["get_demo"];
  };
  "/login": {
    post: operations["log_in"];
  };
  "/ping": {
    get: operations["ping"];
  };
  "/releases": {
    /**
     * Retrieves all releases. 
     * @description Retrieves all releases.
     */
    get: operations["get_releases"];
  };
  "/releases/{name}": {
    /**
     * Retrieves one release identified by `name`. 
     * @description Retrieves one release identified by `name`.
     */
    get: operations["get_release"];
  };
  "/stacklets": {
    /**
     * Retrieves all stacklets. 
     * @description Retrieves all stacklets.
     */
    get: operations["get_stacklets"];
  };
}

export type webhooks = Record<string, never>;

export interface components {
  schemas: {
    /** @description This struct describes a demo with the v2 spec */
    DemoSpecV2: {
      /** @description A short description of the demo */
      description: string;
      /** @description An optional link to a documentation page */
      documentation?: string | null;
      /** @description A variable number of labels (tags) */
      labels?: (string)[];
      /** @description A variable number of Helm or YAML manifests */
      manifests?: (components["schemas"]["ManifestSpec"])[];
      /** @description A variable number of supported parameters */
      parameters?: (components["schemas"]["Parameter"])[];
      /** @description The name of the underlying stack */
      stackableStack: string;
    };
    ManifestSpec: OneOf<[{
      helmChart: string;
    }, {
      plainYaml: string;
    }]>;
    ObjectMeta: {
      name: string;
      namespace: string;
    };
    /** @description Parameter descibes a common parameter format. This format is used in demo and stack definitions */
    Parameter: {
      /** @description Parameter default value */
      default: string;
      /** @description Parameter description */
      description: string;
      /** @description Parameter name */
      name: string;
    };
    ReleaseSpec: {
      /** @description A short description of this release */
      description: string;
      products: components["schemas"]["IndexMap"];
      /** @description Date this released was released */
      releaseDate: string;
    };
    Session: {
      sessionToken: components["schemas"]["SessionToken"];
    };
    SessionToken: string;
    Stacklet: {
      metadata: components["schemas"]["ObjectMeta"];
      product: string;
    };
  };
  responses: never;
  parameters: never;
  requestBodies: never;
  headers: never;
  pathItems: never;
}

export type external = Record<string, never>;

export interface operations {

  /**
   * Retrieves all demos. 
   * @description Retrieves all demos.
   */
  get_demos: {
    responses: {
      /** @description Retrieving a list of demos succeeded */
      200: {
        content: {
          "application/json": (components["schemas"]["DemoSpecV2"])[];
        };
      };
      /** @description Retrieving a list of demos failed */
      404: never;
    };
  };
  /**
   * Retrieves one demo identified by `name`. 
   * @description Retrieves one demo identified by `name`.
   */
  get_demo: {
    parameters: {
      path: {
        _name: string;
      };
    };
    responses: {
      /** @description Retrieving the demo with 'name' succeeded */
      200: {
        content: {
          "application/json": components["schemas"]["DemoSpecV2"];
        };
      };
      /** @description Retrieving the demo with 'name' failed */
      404: never;
    };
  };
  log_in: {
    responses: {
      200: {
        content: {
          "application/json": components["schemas"]["Session"];
        };
      };
      401: {
        content: {
          "text/plain": string;
        };
      };
    };
  };
  ping: {
    responses: {
      200: {
        content: {
          "text/plain": string;
        };
      };
    };
  };
  /**
   * Retrieves all releases. 
   * @description Retrieves all releases.
   */
  get_releases: {
    responses: {
      /** @description Retrieving a list of releases succeeded */
      200: {
        content: {
          "application/json": (components["schemas"]["ReleaseSpec"])[];
        };
      };
      /** @description Retrieving a list of releases failed */
      404: never;
    };
  };
  /**
   * Retrieves one release identified by `name`. 
   * @description Retrieves one release identified by `name`.
   */
  get_release: {
    parameters: {
      path: {
        _name: string;
      };
    };
    responses: {
    };
  };
  /**
   * Retrieves all stacklets. 
   * @description Retrieves all stacklets.
   */
  get_stacklets: {
    responses: {
      200: {
        content: {
          "application/json": (components["schemas"]["Stacklet"])[];
        };
      };
    };
  };
}
