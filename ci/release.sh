WORKDIR=$(pwd)

# Ensure GITHUB_TOKEN is set
if [ -z "$GITHUB_TOKEN" ]; then
    echo "[ERROR] GITHUB_TOKEN environment variable must be set"
    exit 1
fi

# Input parameters
UPLOAD_URL=${1%%\{*}
PLATFORM=${2:-"x86_64-pc-windows-msvc"}

# Display release script usage
function display_usage() {
    echo "
Usage:
  <sh | bash> release.sh <upload-url> [platform]

Example:
  sh release.sh \"https://uploads.github.com/...\" \"x86_64-pc-windows-msvc\"
"
}

# Ensure UPLOAD_URL is set
if [ -z "$UPLOAD_URL" ]; then
    echo "[ERROR] UPLOAD_URL must be set"
    display_usage
    exit 1
fi

### Add release assets
#
# Given the upload url, built and add the platform-specific binary to the
# created release
#
# Args:
# 1. upload_url: The url to upload the release assets to
# 2. platform:   The platform the release asset is built for
#
# Returns:
# None
function add_release_asset() {
    local upload_url=$1
    local platform=$2

    # Derive the asset file name
    local asset_file_name="fbsim"
    local new_asset_file_name="fbsim-${platform}"
    if [[ "$platform" == *"windows"* ]]; then
        asset_file_name="fbsim.exe"
        new_asset_file_name="fbsim-${platform}.exe"
    fi

    # Rename the asset file
    cp "${WORKDIR}/target/release/${asset_file_name}" \
        "${WORKDIR}/target/release/${new_asset_file_name}"

    # Upload the renamed asset
    create_release_asset \
        "${upload_url}" \
        "${WORKDIR}/target/release/${new_asset_file_name}"
}

### Create release asset
#
# Given a release asset upload URL, upload a release asset at a given path
#
# Args:
# 1. upload_url: The release asset upload URL from the release creation
#                response
# 2. asset_path: The path to the release asset file
#
# Returns:
# The cURL response from the GitHub API for creating the release asset
function create_release_asset() {
    local upload_url=$1
    local asset_path=$2

    # Extract the asset name from the path
    local asset_name=${asset_path##*/}

    # Create the release asset
    local asset_res=$(
        curl -s -L \
            -H "Authorization: Bearer ${GITHUB_TOKEN}"  \
            -H "Content-Type: application/octet-stream" \
            --data-binary "@${asset_path}"  \
            "${upload_url}?name=${asset_name}&label=${asset_name}"
    )
    echo $asset_res
}

add_release_asset "${UPLOAD_URL}" "${PLATFORM}"
