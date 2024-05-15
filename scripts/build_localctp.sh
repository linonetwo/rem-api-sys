#!/bin/bash

# Configuration variables
VERSION="6.7.0"
LOCAL_CTP_PATH="./LocalCTP"
LOCALCTP_SYS_PATH="./crates/localctp-sys/thirdparty/LocalCTP/v_current"

# Run the build script for LocalCTP
echo "Building LocalCTP..."
sh "${LOCAL_CTP_PATH}/buildLinux.sh"

# Ensure the target directory exists
mkdir -p "${LOCALCTP_SYS_PATH}"

# Move the specific version of the shared library
echo "Moving shared library..."
mv "${LOCAL_CTP_PATH}/bin/linux/libthosttraderapi_se_v${VERSION}.so" "${LOCALCTP_SYS_PATH}/libthosttraderapi_se.so"

# Copy the CTP header and XML/DTD files
echo "Copying CTP files from version ${VERSION}..."
cp "${LOCAL_CTP_PATH}/LocalCTP/ctp_file/${VERSION}/"*.h "${LOCALCTP_SYS_PATH}/"
cp "${LOCAL_CTP_PATH}/LocalCTP/ctp_file/${VERSION}/"*.xml "${LOCALCTP_SYS_PATH}/"
cp "${LOCAL_CTP_PATH}/LocalCTP/ctp_file/${VERSION}/"*.dtd "${LOCALCTP_SYS_PATH}/"

echo "Update completed."
