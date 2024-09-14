<script>
    import { invoke } from '@tauri-apps/api'
    let comPorts = []
    let bandRates = [
        9600, 19200, 38400, 57600, 115200,
        128000, 153600, 230400, 256000, 460800,
        921600, 1000000, 1152000
    ];
    let connConfig = {
        port: "",
        baudrate: 115200,
        databits: 8,
        parity: 0,
        stopbits: 1
    };
    let connected = false;

    function connect() {
        invoke('connect', { config: connConfig }).then(data => {
            connected = true;
        }).catch(err => {
            console.log(err);
        })
    }
    function disconnect() {
        invoke('disconnect').then(data => {
            connected = false;
        }).catch(err => {
            console.log(err);
        })
    }
    // 获取串口列表
    function checkAvailablePorts() {
        invoke('available_ports').then(data => {
            comPorts = data;
            if (connConfig.port === "") connConfig.port = data[0];
        });
    }
    checkAvailablePorts()
    setInterval(checkAvailablePorts,1000)
    
    setInterval(() => {
        if (connected) {
            invoke("read_ports").then(data => {console.log(data)})
        }
    },1000)
</script>

<div class="w-5/6">
    <div class="flex justify-around mb-1.5">
        <div class="text-sm">连接设置</div>
        <div class="w-1/2 h-0.5 my-auto bg-gray-100"></div>
    </div>
    <select bind:value={connConfig.port} class="select select-sm select-bordered rounded-md w-full text-xs mb-1.5" >
        {#each comPorts as port}
            <option value={port}> 串口号: {port} </option>
        {/each}
    </select>
    <select bind:value={connConfig.baudrate} class="select select-sm select-bordered rounded-md w-full text-xs mb-1.5">
        {#each bandRates as rate}
            <option value={rate}> 波特率: {rate} </option>
        {/each}
    </select>
    <select class="select select-sm select-bordered rounded-md w-full text-xs mb-1.5">
        <option selected>数据位: 8</option>
    </select>
    <select class="select select-sm select-bordered rounded-md w-full text-xs mb-1.5">
        <option selected>校验位: 0</option>
    </select>
    <select class="select select-sm select-bordered rounded-md w-full text-xs mb-1.5">
        <option selected>停止位: 1</option>
    </select>
    <div class="flex justify-end">
        {#if connected}
            <span class="icon-[tabler--square-check-filled] text-green-400 text-xl m-auto"/>
            <button on:click={disconnect} class="btn btn-sm w-10/12 text-xs bg-gray-200 rounded-md">断开连接</button>
        {:else}
            <span class="icon-[tabler--square-x-filled] text-red-400 text-xl m-auto"/>
            <button on:click={connect} class="btn btn-sm w-10/12 text-xs bg-gray-200 rounded-md">连接</button>
        {/if}
    </div>
</div>