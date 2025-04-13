module mu0 (
    input clk,
    input overrideMemControl,
    input overrideMemRnW,
    input [15:0] overrideMemAddr,
    input [15:0] overrideMemDataIn,
    output reg [15:0] overrideMemDataOut,
    output [5:0] led
);
    reg memRq;
    reg readNotWrite;
    reg [15:0] addr;
    reg [15:0] dataIn;
    wire [15:0] dataOut;

    memory mem (
        .memRq(memRq),
        .readNotWrite(readNotWrite),
        .addr(addr),
        .dataIn(dataIn),
        .dataOut(dataOut),
        .led(led)
    );

    always @(posedge clk) begin
        if (overrideMemControl) begin
            memRq <= 1'b1;
            readNotWrite <= overrideMemRnW;
            addr <= overrideMemAddr;
            dataIn <= overrideMemDataIn;
            overrideMemDataOut <= dataOut;
        end else begin
            memRq <= 1'b0;
        end
    end
endmodule
