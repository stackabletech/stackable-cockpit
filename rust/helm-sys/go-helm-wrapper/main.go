package main

// #cgo linux LDFLAGS: -Wl,-unresolved-symbols=ignore-all
// #cgo darwin LDFLAGS: -Wl,-undefined,dynamic_lookup
/*
#include <stdlib.h>
*/
import "C"

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
	"unsafe"

	gohelm "github.com/mittwald/go-helm-client"
	"helm.sh/helm/v3/pkg/action"
	"helm.sh/helm/v3/pkg/repo"

	// Needed for authentication against clusters, e.g. GCP
	// see https://github.com/kubernetes/client-go/issues/242
	_ "k8s.io/client-go/plugin/pkg/client/auth"
)

const HELM_ERROR_PREFIX = "ERROR:"

type Release struct {
	Name        string `json:"name"`
	Version     string `json:"version"`
	Namespace   string `json:"namespace"`
	Status      string `json:"status"`
	LastUpdated string `json:"lastUpdated"`
}

func main() {

}

//export go_install_helm_release
func go_install_helm_release(releaseName *C.char, chartName *C.char, chartVersion *C.char, valuesYaml *C.char, namespace *C.char, suppressOutput bool) *C.char {
	helmClient := getHelmClient(namespace, suppressOutput)

	timeout, _ := time.ParseDuration("10m")
	chartSpec := gohelm.ChartSpec{
		ReleaseName: C.GoString(releaseName),
		ChartName:   C.GoString(chartName),
		Version:     C.GoString(chartVersion),
		ValuesYaml:  C.GoString(valuesYaml),
		Namespace:   C.GoString(namespace),
		UpgradeCRDs: true,
		Wait:        true,
		Timeout:     timeout,
	}

	if _, err := helmClient.InstallChart(context.Background(), &chartSpec, nil); err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	return C.CString("")
}

//export go_uninstall_helm_release
func go_uninstall_helm_release(releaseName *C.char, namespace *C.char, suppressOutput bool) *C.char {
	helmClient := getHelmClient(namespace, suppressOutput)

	if err := helmClient.UninstallReleaseByName(C.GoString(releaseName)); err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	return C.CString("")
}

//export go_helm_release_exists
func go_helm_release_exists(releaseName *C.char, namespace *C.char) bool {
	helmClient := getHelmClient(namespace, true)

	release, _ := helmClient.GetRelease(C.GoString(releaseName))
	return release != nil
}

// Returning a JSON document as GoSlices (array) of objects was a nightmare to
// share between Go and Rust. We also introduce magic return values here. Any
// non-empty result string starting with 'ERROR:' will be treated as an error
// by the Rust code and it will abort operations.
//
//export go_helm_list_releases
func go_helm_list_releases(namespace *C.char) *C.char {
	helmClient := getHelmClient(namespace, true)

	// List all releases, not only the deployed ones (e.g. include pending installations)
	releases, err := helmClient.ListReleasesByStateMask(action.ListAll)
	if err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	var result = make([]Release, len(releases))
	for i, release := range releases {
		result[i] = Release{
			Name:        release.Name,
			Version:     release.Chart.Metadata.Version,
			Namespace:   release.Namespace,
			Status:      release.Info.Status.String(),
			LastUpdated: release.Info.LastDeployed.String(),
		}
	}

	json, err := json.Marshal(result)
	if err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	return C.CString(string(json))
}

// Adds a Helm repo to the temporary repositories file. We also introduce
// magic return values here. Any non-empty result string starting with
// 'ERROR:' will be treated as an error by the Rust code and it will abort
// operations.
//
//export go_add_helm_repo
func go_add_helm_repo(name *C.char, url *C.char) *C.char {
	helmClient := getHelmClient(C.CString("default"), true) // Namespace doesn't matter

	chartRepo := repo.Entry{
		Name: C.GoString(name),
		URL:  C.GoString(url),
	}

	if err := helmClient.AddOrUpdateChartRepo(chartRepo); err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	return C.CString("")
}

//export free_go_string
func free_go_string(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

func getHelmClient(namespace *C.char, suppressOutput bool) gohelm.Client {
	options := gohelm.Options{
		Namespace: C.GoString(namespace),
		Debug:     false,
	}

	if suppressOutput {
		options.DebugLog = func(format string, v ...interface{}) {}
	}

	helmClient, err := gohelm.New(&options)
	if err != nil {
		panic(err)
	}

	return helmClient
}
