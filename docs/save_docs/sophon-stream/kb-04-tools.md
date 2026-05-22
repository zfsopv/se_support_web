# Sophon-Stream 工具 (Tools) 参考文档

> 本文档由 tools/ 目录下所有中文 Markdown 文档合并而成。
> 生成时间: 2026-05-22 15:04:00

## stream-agent

# stream-agent
## 目录
- [stream-agent](#stream-agent)
  - [目录](#目录)
  - [1.简介](#1简介)
    - [1.1 代码目录结构说明](#11-代码目录结构说明)
  - [2.快速开始](#2快速开始)
    - [2.1 安装application-web](#21-安装application-web)
    - [2.2 启动stream-agent](#22-启动stream-agent)
    - [2.3 运行算法任务](#23-运行算法任务)
  - [3.server.py说明](#3serverpy说明)
    - [3.1 接口文档](#31-接口文档)
    - [3.2 接口实现说明](#32-接口实现说明)
      - [3.2.1 创建任务](#321-创建任务)
      - [3.2.2 删除任务](#322-删除任务)
      - [3.2.3 任务列表查询](#323-任务列表查询)
      - [3.2.4 处理stream结果并发送到application-web](#324-处理stream结果并发送到application-web)
## 1.简介
stream-agent是sophon-stream代理应用，使用python开发了一套基于sophon-stream的应用。对外开放http接口，实现sophon-stream任务启动、停止、配置等功能。配合[application-web](https://github.com/sophgo/application-web)可实现一套部署于算能SoC设备中的算法应用。  
stream-agent框架如图所示：  
![Alt text](pics/struct.png)  
其中application-web是算法应用软件，包括算法任务配置、告警图片展示等功能；通过4个http接口对接stream-agent。

### 1.1 代码目录结构说明
```bash
├── common.py #基础函数，如拷贝目录，检测文件
├── config_algorithm.py #算法配置文件，调用samples中的python函数
├── samples
│   ├── license_area_intrusion #车牌识别例程
│   │   ├── config #启动stream的基础配置文件，启动任务时会拷贝config文件内容到tasks目录，并修改每个json配置中文件位置
│   │   │   ├── converger.json
│   │   │   ├── decode.json
│   │   │   ├── distributor_time_class.json
│   │   │   ├── engine_group.json
│   │   │   ├── filter.json
│   │   │   ├── http_push.json
│   │   │   ├── license_area_intrusion_demo.json
│   │   │   ├── lprnet_group.json
│   │   │   └── yolov5_group.json
│   │   └── license_area_intrusion.py #车牌识别算法处理，包括修改config配置文件和接受stream结果并上报函数
│   ├── openpose #人体pose检测（关键点识别）
│   ├── tripwire #人员越线检测
│   ├── yolov5 #yolov5例程
│   └── yolov8 #yolov8例程
├── server.py #运行主程序
└── start_server.sh #启动脚本
```
## 2.快速开始
>  **运行环境：** 算能SoC设备  

> **注意：**  
1. 在启动之前，您需要将编译好的`sophon-stream`完整的拷贝到SoC设备
2. 跑通sophon-stream的samples中部分应用，如yolov5例程，知晓启动方式以及json文件如何配置等。如果需要启动yolov5算法应用，需要在sophon-stream/samples/yolov5下载好对应的模型等数据。   
3. 下面所有安装都是在SoC设备进行    

### 2.1 安装application-web  
```bash
pip3 install dfss -i https://pypi.tuna.tsinghua.edu.cn/simple --upgrade
python3 -m dfss --url=open@sophgo.com:/sophon-stream/tools/application-web-linux_arm64.tgz
tar -xzvf application-web-linux_arm64.tgz
cd application_web
sudo ./install.sh
```
安装完成后，使用浏览器输入`http://{ip}:8089`，打开页面，ip为SoC平台设备ip地址。用户名和密码均为`admin`  

### 2.2 启动stream-agent
1. 修改`stream-agent/samples`目录中config内容，以车牌识别例程(license_area_intrusion)为例  
license_area_intrusion内容如下：
``` bash
.
├── config
│   ├── converger.json
│   ├── decode.json
│   ├── distributor_time_class.json
│   ├── engine_group.json
│   ├── filter.json
│   ├── http_push.json
│   ├── license_area_intrusion_demo.json
│   ├── lprnet_group.json
│   └── yolov5_group.json
└── license_area_intrusion.py
```
根据设备类型修改`yolov5_group.json`和`lprnet_group.json`配置中"model_path"的值；以`yolov5_group.json`为例，初始值为
`"{Stream_path}/samples/license_area_intrusion/models/yolov5s-licensePLate/BM1684X/yolov5s_v6.1_license_3output_int8_1b.bmodel"`,其中`{Stream_path}`为sophon-stream目录位置，无需改动；若SoC设备是SE9-16，那么就需要修改为BM1688的模型，如`"{Stream_path}/samples/license_area_intrusion/models/yolov5s-licensePLate/BM1688/yolov5s_v6.1_license_3output_int8_1b_2core.bmodel"`  
如果您需要运行其它模型，yolov5、yolov8等，则修改`stream-agent/samples`中对应config配置中模型的路径。  

2. 安装python依赖
```bash
pip3 install  Pillow -i https://pypi.tuna.tsinghua.edu.cn/simple
```  
3. 运行stream-agent
```bash
cd tools/stream-agent #进入stream-agent目录
./start_server.sh
``` 
### 2.3 运行算法任务
1. 浏览器打开application-web的任务管理页面  
![Alt text](pics/image.png)
2. 添加算法任务  
![Alt text](pics/add_task.png)
3. 启动算法任务  
![Alt text](pics/start_task.png)
4. 查看告警  
![Alt text](pics/alarm_res.png)

## 3.server.py说明
### 3.1 接口文档
[stream-agent接口说明](./docs/stream-agent.md)
### 3.2 接口实现说明
#### 3.2.1 创建任务  
![Alt text](pics/start.png)  
在`build_start(data)`函数中实现，首先在在tasks文件夹中根据任务名称创建任务文件（如task_1001），然后获取下发的算法类型比如yolov5，将`stream-agent/samples/yolov5/config`文件拷贝到task_1001；  
将`task_1001/config`文件中的各个json文件进行配置，如替换`{Stream_path}`为sophon-stream的实际地址，检查json文件中模型文件是否存在等；  
调用`samples/yolov5/yolov5.py中yolov5_build_config`函数，修改demo.json和group.json对应数据
在task_1001目录启动sophon-stream子进程，sophon-stream输出的日志保存在`task_1001_stream.log`，加入`process_pools`监测  
#### 3.2.2 删除任务
根据任务ID停止process_pools中的子进程，删除taskId_map中任务数据
#### 3.2.3 任务列表查询
从taskId_map中获取任务状态，返回任务列表
#### 3.2.4 处理stream结果并发送到application-web
sophon-stream运行结果是通过http的方式发送到stream-agent的/alarm/rev接口   
stream的http插件配置：
```json
{
  "configure": {
    "ip": "127.0.0.1",
    "port": 8001,
    "path": "/alarm/rev"
  },
  "shared_object": "{Stream_path}/build/lib/libhttp_push.so",
  "name": "http_push",
  "side": "sophgo",
  "thread_number": 1
}
```
处理该接口时，必须明确sophon-stream的http上报结果格式，每种算法上报的格式都不一样，获取上报数据格式可参考[application-web工具使用](../../samples/structured_recognition/README.md#6)；  
下面是车牌识别算法上报的结果。其中`mDetectedObjectMetadatas`是yolov5检测的结果，`mSubObjectMetadatas`是基于yolov5车牌检测进行识别的结果，是一一对应的。`"mSpData"`是图片base64编码，此处省略内容。
```json
{
    "mDetectedObjectMetadatas": [
        {
            "mBox": {
                "mHeight": 84,
                "mWidth": 357,
                "mX": 222,
                "mY": 408
            },
            "mClassify": 0,
            "mScores": [
                0.9197901487350464
            ]
        }
    ],
    "mFps": 0.0,
    "mFrame": {
        "mChannelId": 0,
        "mEndOfStream": false,
        "mFrameId": 10,
        "mHeight": 1024,
        "mSpData": "/9j/2wBDAA7VAAAAAAA==",
        "mTimestamp": 1721992917985880,
        "mWidth": 1920
    },
    "mGraphId": 0,
    "mSubId": 0,
    "mSubObjectMetadatas": [
        {
            "mFps": 0.5036436915397644,
            "mFrame": {
                "mChannelId": 0,
                "mEndOfStream": false,
                "mFrameId": 10,
                "mHeight": 0,
                "mSpData": "/9jAAA",
                "mTimestamp": 0,
                "mWidth": 0
            },
            "mGraphId": -1105730188,
            "mRecognizedObjectMetadatas": [
                {
                    "mLabelName": "皖AD10005",
                    "mScores": [],
                    "mTopKLabels": []
                }
            ],
            "mSubId": 1
        }
    ]
}
```
处理流程：  
![Alt text](pics/client.png)  
首先根据`"mChannelId"`从`taskId_map`中获取`taskId`等信息，然后根据算法类型（如yolov5），进入`samples/yolov5/yolov5.py`的`yolov5_trans_json(json_data, task_id, Type)`函数
将yolov5的结果进行裁剪，进行base64编码
上报到application-web
### stream-agent 接口文档

## /stream-agent/创建任务

#### 接口URL
> http://{{host}}/task/create

#### 请求方式
> POST

#### Content-Type
> json

#### 请求Header参数
参数名 | 示例值 | 参数类型 | 是否必填 | 参数描述
--- | --- | --- | --- | ---
Content-Type | application/json;charset=UTF-8 | String | 是 | -
#### 请求Body参数
```javascript
{
  "TaskID": "1001",
  "InputSrc": {
    "SrcID": "test",
    "StreamSrc": {
      "Address": "/opt/sophon/area.mp4"
    }
  },
  "Algorithm": [
    {
      "Type": 2,
      "TrackInterval": 1,
      "DetectInterval": 150,
      "AlarmInterval": 1,
      "threshold": 50,
      "TargetSize": {
        "MinDetect": 30,
        "MaxDetect": 250
      },
      "DetectInfos": [
        {
          "TripWire": {
            "LineStart": {
              "X": 0,
              "Y": 0
            },
            "LineEnd": {
              "X": 0,
              "Y": 0
            },
            "DirectStart": {
              "X": 0,
              "Y": 0
            },
            "DirectEnd": {
              "X": 0,
              "Y": 0
            }
          },
          "HotArea": [
            {
              "X": 0,
              "Y": 0
            },
            {
              "X": 1920,
              "Y": 0
            },
            {
              "X": 1920,
              "Y": 1024
            },
            {
              "X": 0,
              "Y": 1024
            }
          ]
        }
      ]
    }
  ],
  "Reporting": {
    "ReportUrlList": [
      "http://172.28.8.86:8089/api/upload"
    ]
  }
}
```
参数名 | 示例值 | 参数类型 | 是否必填 | 参数描述
--- | --- | --- | --- | ---
TaskID | 1001 | String | 是 | 任务名称
InputSrc | - | Object | 是 | -
InputSrc.SrcID | test | String | 是 | 视频源名称
InputSrc.StreamSrc | - | Object | 是 | -
InputSrc.StreamSrc.Address | /opt/sophon/area.mp4 | String | 是 | 视频地址，视频文件绝对路径或者rtsp流等
Algorithm | - | Array | 是 | -
Algorithm.Type | 2 | Integer | 是 | 算法类型：{1: "tripwire", 2: "license_area_intrusion", 3: "yolov5", 4: "yolov8", 5: "openpose"}
Algorithm.TrackInterval | 1 | Integer | 是 | 追踪间隔
Algorithm.DetectInterval | 150 | Integer | 是 | 检测间隔（每150帧检测一次）
Algorithm.AlarmInterval | 1 | Integer | 是 | 告警间隔
Algorithm.threshold | 50 | Integer | 是 | 阈值（1-100）
Algorithm.TargetSize | - | Object | 是 | -
Algorithm.TargetSize.MinDetect | 30 | Integer | 是 | -
Algorithm.TargetSize.MaxDetect | 250 | Integer | 是 | -
Algorithm.DetectInfos | - | Array | 是 | -
Algorithm.DetectInfos.TripWire | - | Object | 是 | 人员越线检测的检测线
Algorithm.DetectInfos.TripWire.LineStart | - | Object | 是 | -
Algorithm.DetectInfos.TripWire.LineStart.X | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.LineStart.Y | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.LineEnd | - | Object | 是 | -
Algorithm.DetectInfos.TripWire.LineEnd.X | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.LineEnd.Y | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.DirectStart | - | Object | 是 | -
Algorithm.DetectInfos.TripWire.DirectStart.X | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.DirectStart.Y | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.DirectEnd | - | Object | 是 | -
Algorithm.DetectInfos.TripWire.DirectEnd.X | 0 | Integer | 是 | -
Algorithm.DetectInfos.TripWire.DirectEnd.Y | 0 | Integer | 是 | -
Algorithm.DetectInfos.HotArea | - | Array | 是 | 检测区域
Algorithm.DetectInfos.HotArea.X | 0 | Integer | 是 | -
Algorithm.DetectInfos.HotArea.Y | 0 | Integer | 是 | -
Reporting | - | Object | 是 | -
Reporting.ReportUrlList | http://172.28.8.86:8089/api/upload | Array | 是 | 告警上报地址

#### 成功响应示例
```javascript
{
	"Code": 0,
	"Msg": "success"
}
```
参数名 | 示例值 | 参数类型 | 参数描述
--- | --- | --- | ---
Code | 0 | Integer | 0表示成功，其他表示失败
Msg | success | String | -
#### 错误响应示例
```javascript
{
    "code": 1,
    "msg": "失败原因"
}
```
参数名 | 示例值 | 参数类型 | 参数描述
--- | --- | --- | ---
code | 1 | Integer | -
msg | 失败原因 | String | -
## /stream-agent/查询任务列表


#### 接口URL
> http://{{host}}/task/list

#### 请求方式
> POST

#### Content-Type
> json

#### 请求Header参数
参数名 | 示例值 | 参数类型 | 是否必填 | 参数描述
--- | --- | --- | --- | ---
Content-Type | application/json;charset=UTF-8 | String | 是 | -
#### 请求Body参数
```javascript
{
}
```
#### 认证方式
```text
noauth
```
#### 预执行脚本
```javascript
暂无预执行脚本
```
#### 后执行脚本
```javascript
暂无后执行脚本
```
#### 成功响应示例
```javascript
{"Code":0,"Msg":"success","Result":[{"Status":1,"TaskID":"1001"}]}
```
参数名 | 示例值 | 参数类型 | 参数描述
--- | --- | --- | ---
Code | 0 | Integer | 0表示成功，其他表示失败
Msg | success | String | -
Result | - | Array | -
Result.Status | 1 | Integer | 1表示任务运行中，0表示任务停止
Result.TaskID | 1001 | String | 任务名称
#### 错误响应示例
```javascript
{
    "code": 1,
    "msg": "失败原因"
}
```
参数名 | 示例值 | 参数类型 | 参数描述
--- | --- | --- | ---
code | 1 | Integer | -
msg | 失败原因 | String | -
## /stream-agent/删除任务
```text
根据开始时间、结束时间、视频通道、算法类型对告警信息进行检索，显示告警图片和告警信息
```
#### 接口状态
> 已完成

#### 接口URL
> http://{{host}}/task/delete

#### 请求方式
> POST

#### Content-Type
> json

#### 请求Header参数
参数名 | 示例值 | 参数类型 | 是否必填 | 参数描述
--- | --- | --- | --- | ---
Content-Type | application/json;charset=UTF-8 | String | 是 | -
#### 请求Body参数
```javascript
{
   "TaskID": "1001"
}
```
参数名 | 示例值 | 参数类型 | 是否必填 | 参数描述
--- | --- | --- | --- | ---
TaskID | 1001 | String | 是 | 任务名称

#### 成功响应示例
```javascript
{"Code":0,"Msg":"success"}
```
参数名 | 示例值 | 参数类型 | 参数描述
--- | --- | --- | ---
Code | 0 | Integer | 0表示成功，其他表示失败
Msg | success | String | -
#### 错误响应示例
```javascript
{
    "code": 1,
    "msg": "失败原因"
}
```
参数名 | 示例值 | 参数类型 | 参数描述
--- | --- | --- | ---
code | 1 | Integer | -
msg | 失败原因 | String | -
---

## stress (压测脚本)

# 压测脚本使用说明

## 说明

* 本目录下的`stress.sh`和`get_stress_metric.py`文件为sophon-stream在soc模式下进行压测的文件，pcie模式无法运行。`stress.sh`会根据参数运行某个例程并保存运行过程中的算法、设备、系统信息；`get_stress_metric.py`会从保存的信息中统计出如cpu利用率、fps等各项指标。

* 使用前，推荐切换到root用户，以保证有足够的文件权限。 

## 1. stress.sh

使用方法：

```bash
# ./stress.sh <sample path> <sample log name> <tpu log name> <cpu log name> <npu_vpp_vpu log name> <inputs and threads>
./stress.sh ../../samples/yolov5/build/yolov5_demo alg.log tpu.log host.log dev.log 1111
```

上述命令中的最后一项参数是例程的输入路数和算法线程数，例如：一路输入，前处理、推理、后处理各一个线程的情况，可以写作`1111`。

该脚本不会根据参数中的输入路数和算法线程数来修改对应例程的配置文件，因此，最后一项参数实际上只是起到了标识作用。实际使用中，请注意最后一项参数与实际配置一致。

运行结束后，会在当前目录下生成保存了所有日志信息的子目录。

## 2. get_stress_metric.py

使用方法：

```bash
# python3 get_stress_metric.py --alg_log <sample log path> --tpu_log <tpu log path> --host_log <cpu log path> --dev_log <npu_vpp_vpu log path> --channel_combination <inputs and threads>
python3 get_stress_metric.py --alg_log ./yolov5_demo/1111/alg.log --tpu_log ./yolov5_demo/1111/tpu.log --host_log ./yolov5_demo/1111/host.log --dev_log ./yolov5_demo/1111/dev.log --channel_combination 1111
```

该脚本以`stress.sh`输出的日志文件为输入，依据其计算多项指标。

该脚本输出结果为markdown格式的表格，参考结果如下：

![](../../docs/pics/stress.png)


---

## visualize (可视化工具)

# visualize

## 1 简介

visualize是针对sophon-stream项目运行的可视化工具。本工具能够将项目的运行结果在前端可视化，使得用户能够实时了解到项目的推理结果。通过该工具，用户可以在前端实时查看项目的推理结果，直观了解每个阶段的处理情况和输出。这种交互性的可视化方式为项目的调试和优化提供了极大的便利，使得非技术人员也能轻松使用。

项目总共包含两个子目录，一个子目录是server，为项目的后端程序，另外一个子目录是web_ui，为项目的前端程序。想要使用本项目，首先需要启动web_server后端程序，然后再启动web_ui前端程序，具体启动方法，请查看后续的快速入门文档。

## 2 功能介绍
![项目页面](./web_ui/interface.jpg)

visualize可视化工具为用户提供以下几种功能：

   - 图形化的操作和配置界面：

     我们为每个pipeline都提供了内置的pipeline示例配置，您可以直接运行示例，快速体验sophon-stream。
   - 支持修改不同pipeline参数：

     我们支持在线配置pipeline的各种参数，您可以直接点击选择不同的配置文件，在线提交。
   - 支持预览pipeline运行结果：

     支持预览实时运行中的视频结果；

## 3 使用方法
使用本项目工具，请依次启动`server`后端程序和`ui`前端程序，请依次参考下列入门使用方法：

首先请参考后端程序的入门使用方法，将后端程序开启，请参考[server用户文档](./server/README.md)

然后请参考前端程序的入门使用方法，将前端程序开启，请参考[web_ui用户文档](./web_ui/README.md)

### visualize - server

# web_server

## 1 简介

web_server后端开发程序，主要包含五个核心功能模块：获取pipeline List的GET方法、针对指定pipeline id的GET方法、针对指定pipeline id的PATCH方法、针对指定json id的GET方法、以及针对指定json id的PUT方法，这五个模块组成了web_server后端开发程序的核心功能。通过这些接口，前端或其他客户端可以与后端进行交互，实现对pipeline和json数据的管理与算能demo程序的启停操作。

web_server后端开发程序的这五个模块主要内容包括：

1. 获取pipeline List的GET方法：
   - HTTP方法：GET
   - URL格式： `http://{ip address}:{port}/pipelines`
   - 描述：该模块用于获取所有pipeline的摘要信息。
   - 功能：客户端可以使用此请求获取所有可用的pipeline的摘要信息列表，包括所有pipeline的ID、名称、状态以及对应的json列表等完整内容。

2. 针对指定pipeline id的GET方法：
   - HTTP方法：GET
   - URL格式： `http://{ip address}:{port}/pipelines/{pipeline_id}`
   - 描述：该模块用于从服务器获取指定pipeline ID的详细信息。
   - 功能：客户端可以使用此请求获取特定pipeline ID的所有属性和当前状态，主要有以下四个属性：pipeline_id，pipeline_name，is_running，json_list，该方法得到的数据内容是 [pipeline List GET] 得到的数据的一个实例。

3. 针对指定pipeline id的PATCH方法：
   - HTTP方法：PATCH
   - URL格式： `http://{ip address}:{port}/pipelines/{pipeline_id}`
   - 描述：该模块用于对pipeline进行部分更新，允许客户端向服务器发送部分更改的请求。
   - 功能：支持对pipeline的指定字段进行更新操作，如更新pipeline的状态 is_runnning。本项目目前仅用于修改pipeline的is_running状态，而不需要重新发送整个pipeline的数据。
   
4. 针对指定json id的GET方法：
   - HTTP方法：GET
   - URL格式： `http://{ip address}:{port}/jsons/{json_id}`
   - 描述：该模块用于获取json数据的详细信息。
   - 功能：允许客户端请求服务器返回一个特定json_id文件内json数据的详细信息，可以用于获取json数据的元数据或其他相关信息。

5. 针对指定json id的PUT方法：
   - HTTP方法：PUT
   - URL格式： `http://{ip address}:{port}/jsons/{json_id}`
   - 描述：该模块用于向服务器上传（或更新）json数据。
   - 功能：客户端可以使用此请求将一个json数据发送到服务器，从而创建新的json数据或更新已有的json数据。

以上五个模块是`web_server`后端开发程序的核心功能，通过这些接口，前端或其他客户端可以与后端进行交互，实现对`pipeline`和`json`数据的管理与操作。请注意，这里的描述仅为示例，实际的API设计可能需要更多的参数和验证措施，以确保数据的安全性和完整性。

## 2 使用方法

在编译程序前，请确认您已使用本项目提供的 `scripts/gen_jsons.py` 脚本，一键生成 `all_files_preview.json` 项目的配置信息索引文件。

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。测试的参数及运行方式是一致的，下面分别以PCIe和SoC模式进行介绍。
### 2.1 PCIe模式编译及运行方法
   ```bash
   mkdir build && cd build
   cmake ..
   make -j
   ./sever
   ```
### 2.2 SoC模式编译及运行方法
   ```bash
   mkdir build && cd build
   cmake ../ -DTARGET_ARCH=soc -DSOPHON_SDK_SOC=/path/to/soc-sdk
   make -j
   ./sever
   ```

>**注意：**

soc环境运行时如果报错，类似问题均需要设置环境变量：
```bash
./yolov5_demo: error while loading shared libraries: libivslogger.so: cannot open shared object file: No such file or directory
```

需要设置环境变量
```bash
export LD_LIBRARY_PATH=path-to/sophon-stream/build/lib/:$LD_LIBRARY_PATH
```

## 3 输入输出样例

### 3.1 获取pipeline List的GET方法：
   - 输入样例：
   无输入
   - 输出样例：
   ```json
   {
      "data": [
         {
            "is_running": false,
            "json_list": [
               {
                  "json_id": "0",
                  "json_name": "decode.json"
               },
               ......
            ],
            "pipeline_id": 0,
            "pipeline_name": "yolov5"
         },
         ......
      ],
      "message": "get pipeline list success",
      "status": "success"
   }
   ```

### 3.2 针对指定pipeline id的GET方法：
  - 输入样例：
   无输入
  - 输出样例：
   ```json
   {
	"data": [
		{
			"is_running": false,
			"json_list": [
				{
					"json_id": "0",
					"json_name": "decode.json"
				},
				......
			],
			"pipeline_id": 0,
			"pipeline_name": "yolov5"
		}
	],
	"message": "get pipeline id content success",
	"status": "success"
   }
   ```

### 3.3 针对指定pipeline id的PATCH方法：
   - 输入样例：
   ```json
   {
    "is_running":true
   }
   ```
   - 输出样例：
   ```json
   {
      "data": [
         {
            "is_running": true,
            "json_list": [],
            "pipeline_id": 0,
            "pipeline_name": "yolov5"
         }
      ],
      "message": "patch pipeline success",
      "status": "success"
   }
   ```

### 3.4 针对指定json id的GET方法：
   - 输入样例：
   无输入
   - 输出样例：
   ```json
   {
   "data": {
      "json_content": {
         "configure": {},
         "name": "decode",
         "shared_object": "../../../build/lib/libdecode.so",
         "side": "sophgo",
         "thread_number": 1
      },
      "json_id": 0,
      "json_name": "decode.json"
   },
   "message": "get json id content success!!!",
   "status": "success"
   }
   ```

### 3.5 针对指定json id的PUT方法：
   - 输入样例：
   无输入
   - 输出样例：
   ```json
   {
	"data": {
		"json_content": {
			"configure": {},
			"name": "decode",
			"shared_object": "../../../build/lib/libdecode.so",
			"side": "sophgo",
			"thread_number": 1
		},
		"json_id": 0,
		"json_name": "decode.json"
	},
	"message": "put json id content success!!!",
	"status": "success"
   }
   ```



### visualize - web_ui

# Web UI 使用方法

## 1 启动 Web UI
输入下列代码启动程序
```bash
cd build
python3 -m http.server 3000
```

在浏览器中输入下列网址来打开页面
```
http:/{host-ip}:3000/index.html
```

>注意：
>1.前后端服务默认运行在同一台机器，host-ip为服务器地址，3000为浏览器端口，如有占用可更换
>2.此方法可无需npm启动web_ui
>3.如果显示视频画面需要配置encode.json中的"encode_type"为"WS"

## 2 使用react启动
如果需要开发调试，需要安装npm
```bash
sudo apt install npm
```

输入以下命令启动程序
```bash
npm install --legacy-peer-deps
npm start
```

注意：使用react启动本项目依赖node(>v14.17.0)和npm(>v6.14.13)工具，react默认使用3000端口

## 3 Pipeline Json配置说明
### 3.1 yolov5参数配置

yolov5是sophon-stream中一种视频目标检测应用。yolov5例程算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率。

yolov5 demo中各部分参数位于 [config](../../../samples/yolo5/config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine.json                 # sophon-stream graph配置
├── yolov5_demo.json            # yolov5 demo配置
├── yolov5_infer.json           # yolov5 推理配置
├── yolov5_post.json            # yolov5 后处理配置
└── yolov5_pre.json             # yolov5 前处理配置
```
更详细的介绍，请参见 [yolov5的README.md](../../../samples/yolov5/README.md)。

### 3.2 yolox参数配置
yolox是sophon-stream中一种视频目标检测应用。yolox由旷视提出，是基于YOLO系列的改进.
在yolox例程中，yolox算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率。

yolox demo中各部分参数位于 [config](../../../samples/yolox/config/) 目录，结构如下所示：

```bash
./config
├── decode.json             # 解码配置
├── engine.json             # sophon-stream graph配置
├── yolox_demo.json         # yolox demo配置
├── yolox_infer.json        # yolox 推理配置
├── yolox_post.json         # yolox 后处理配置
└── yolox_pre.json          # yolox 前处理配置
```

更详细的介绍，请参见 [yolox的README.md](../../../samples/yolox/README.md)。

### 3.3 resnet参数配置
resnet是sophon-stream中的一种视频目标分类应用。深度残差网络（Deep residual network, ResNet）是由于Kaiming He等在2015提出的深度神经网络结构，它利用残差学习来解决深度神经网络训练退化的问题。

resnet demo中各部分参数位于 [config](../../../samples/resnet/config/) 目录，结构如下所示：

```bash
./config
├── decode.json             # 解码配置
├── engine.json             # sophon-stream graph配置
├── resnet_demo.json        # resnet demo配置
├── resnet_roi.json         # resnet roi配置
└── resnet_classify.json    # resnet 插件配置
```

更详细的介绍，请参见 [resnet的README.md](../../../samples/resnet/README.md)。

### 3.4 bytetrack参数配置
bytetrack是sophon-stream中一种视频目标跟踪应用。ByteTrack是一个简单、快速、强大的多目标跟踪器，且不依赖特征提取模型。

bytetrack demo中各部分参数位于 [config](../../../samples/bytetrack/config/) 目录，结构如下所示：

```bash
./config
   ├── bytetrack_demo.json       # bytetrack demo 配置
   ├── bytetrack.json            # bytetrack目标跟踪器参数配置
   ├── decoder.json              # 解码配置
   ├── engine.json               # sophon-stream graph配置
   ├── infer.json                # 目标检测器推理配置
   ├── post.json                 # 目标检测器后处理配置
   └── pre.json                  # 目标检测器前处理配置
```

更详细的介绍，请参见 [bytetrack的README.md](../../../samples/bytetrack/README.md)。

### 3.5 yolov5_bytetrack_distributor_resnet_converger参数配置
yolov5_bytetrack_distributor_resnet_converger是sophon-stream中一种包含了多算法和按类别发往不同分支的复杂应用。

yolov5_bytetrack_distributor_resnet_converger demo中各部分参数位于 [./config](../../../samples/yolov5_bytetrack_distributor_resnet_converger/config/)目录，结构如下所示：

```bash
./config
├── bytetrack.json                                                    # bytetrack跟踪算法配置
├── converger.json                                                    # 汇聚element配置
├── decode.json                                                       # 解码配置
├── distributor_class.json                                            # 每帧按类别分发
├── distributor_frame_class.json                                      # 跳帧按类别分发
├── distributor_frame.json                                            # 跳帧分发full frame
├── distributor_time_class.json                                       # 间隔时间按类别分发（默认）
├── distributor_time.json                                             # 间隔时间分发full frame
├── engine.json                                                       # graph配置
├── resnet_car.json                                                   # resnet 车辆颜色分类
├── resnet_person.json                                                # resnet 行人性别分类
├── yolov5_bytetrack_distributor_resnet_converger_demo.json           # demo配置
├── yolov5_infer.json                                                 # yolov5 推理配置
├── yolov5_post.json                                                  # yolov5 后处理配置
└── yolov5_pre.json                                                   # yolov5 前处理配置
```

更详细的介绍，请参见 [yolov5_bytetrack_distributor_resnet_converger的README.md](../../../samples/yolov5_bytetrack_distributor_resnet_converger/README.md)。

### 3.6 yolox_bytetrack_osd_encode参数配置
yolox_bytetrack_osd_encode是sophon-stream中一种视频目标检测跟踪应用，最后将算法结果推流输出。

yolox_bytetrack_osd_encode demo中各部分参数位于 [./config](../../../samples/yolox_bytetrack_osd_encode/config/)目录，结构如下所示：

```bash
./config
├── bytetrack.json                                                    # bytetrack跟踪算法配置
├── decode.json                                                       # 解码配置
├── encode.json                                                       # 编码配置
├── engine.json                                                       # graph配置
├── osd.json                                                          # 对具体某个element的配置细节
├── ws.json                                                           # websocket配置
├── yolox_bytetrack_osd_encode_demo.json                              # demo配置
├── yolox_infer.json                                                  # yolox 推理配置
├── yolox_post.json                                                   # yolox 后处理配置
├── yolox_pre.json                                                    # yolox 前处理配置
```

更详细的介绍，请参见 [yolox_bytetrack_osd_encode的README.md](../../../samples/yolox_bytetrack_osd_encode/README.md)。

---

## video-vue

安装node.js
```bash
# installs nvm (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
# download and install Node.js (you may need to restart the terminal)
nvm install 20
# verifies the right Node.js version is in the environment
node -v # should print `v20.15.1`
# verifies the right NPM version is in the environment
npm -v # should print `10.7.0`
```

安装依赖
```bash
npm install
```

启动服务
```bash
npm run dev
```