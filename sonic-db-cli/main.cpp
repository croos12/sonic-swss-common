#include "sonic-db-cli.h"
#include "common/dbconnector.h"
#include <iostream>
#include <string>
#include <sstream>

using namespace swss;
using namespace std;

int main(int argc, char** argv)
{
    auto initializeGlobalConfig = []()
    {
        SonicDBConfig::initializeGlobalConfig(SonicDBConfig::DEFAULT_SONIC_DB_GLOBAL_CONFIG_FILE);
    };

    auto initializeConfig = [](const string& dpu_name = "")
    {
        if(dpu_name.empty()){
            SonicDBConfig::initialize(SonicDBConfig::DEFAULT_SONIC_DB_CONFIG_FILE);
        }
        else{
            std::stringstream path_stream;
            path_stream << DPU_SONIC_DB_CONFIG_PATH_PREFIX << dpu_name << DPU_SONIC_DB_CONFIG_PATH_SUFFIX;
            auto path = path_stream.str();
            SonicDBConfig::initialize(path);
        }
    };

    return cli_exception_wrapper(
                    argc,
                    argv,
                    initializeGlobalConfig,
                    initializeConfig);
}