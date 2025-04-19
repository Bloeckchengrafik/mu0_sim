module mu0 (
    input clk,
    input overrideMemControl,
    input overrideMemRnW,
    input [15:0] overrideMemAddr,
    input [15:0] overrideMemDataIn,
    output reg [15:0] overrideMemDataOut,
    output [5:0] led
);
    reg memRq = 0;
    reg readNotWrite = 1;
    reg [15:0] addr = 0;
    reg [15:0] dataIn = 0;

    wire _memRq;
    wire _readNotWrite;
    wire [15:0] _addr;
    wire [15:0] _dataIn;
    wire [15:0] dataOut;

    memory mem (
        .clk(clk),
        .memRq(_memRq),
        .readNotWrite(_readNotWrite),
        .addr(_addr),
        .dataIn(_dataIn),
        .dataOut(dataOut),
        .led(led)
    );

    assign overrideMemDataOut = dataOut;
    assign _memRq = overrideMemControl ? 1 : memRq;
    assign _readNotWrite = overrideMemControl ? overrideMemRnW : readNotWrite;
    assign _addr = overrideMemControl ? overrideMemAddr : addr;
    assign _dataIn = overrideMemControl ? overrideMemDataIn : dataIn;
endmodule
