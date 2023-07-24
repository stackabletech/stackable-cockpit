package main

import (
	"C"
	"context"
	"encoding/json"
	"fmt"
	"time"

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
func go_install_helm_release(releaseName string, chartName string, chartVersion string, valuesYaml string, namespace string, suppressOutput bool) *C.char {
	helmClient := getHelmClient(namespace, suppressOutput)

	timeout, _ := time.ParseDuration("10m")
	chartSpec := gohelm.ChartSpec{
		ReleaseName: releaseName,
		ChartName:   chartName,
		Version:     chartVersion,
		ValuesYaml:  valuesYaml,
		Namespace:   namespace,
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
func go_uninstall_helm_release(releaseName string, namespace string, suppressOutput bool) *C.char {
	helmClient := getHelmClient(namespace, suppressOutput)

	if err := helmClient.UninstallReleaseByName(releaseName); err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	return C.CString("")
}

//export go_helm_release_exists
func go_helm_release_exists(releaseName string, namespace string) bool {
	helmClient := getHelmClient(namespace, true)

	release, _ := helmClient.GetRelease(releaseName)
	return release != nil
}

// Returning a JSON document as GoSlices (array) of objects was a nightmare to
// share between Go and Rust. We also introduce magic return values here. Any
// non-empty result string starting with 'ERROR:' will be treated as an error
// by the Rust code and it will abort operations.
//
//export go_helm_list_releases
func go_helm_list_releases(namespace string) *C.char {
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
func go_add_helm_repo(name string, url string) *C.char {
	helmClient := getHelmClient("default", true) // Namespace doesn't matter

	chartRepo := repo.Entry{
		Name: name,
		URL:  url,
	}

	if err := helmClient.AddOrUpdateChartRepo(chartRepo); err != nil {
		return C.CString(fmt.Sprintf("%s%s", HELM_ERROR_PREFIX, err))
	}

	return C.CString("")
}

func getHelmClient(namespace string, suppressOutput bool) gohelm.Client {
	options := gohelm.Options{
		Namespace: namespace,
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
