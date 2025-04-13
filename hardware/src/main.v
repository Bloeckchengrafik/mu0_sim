module main (
    input clk,
    input uart_rx,
    output uart_tx,
    output reg [5:0] led,
    input btn1
);

    wire overrideMemControl;
    wire overrideMemRnW;
    wire [15:0] overrideMemAddr;
    wire [15:0] overrideMemDataIn;
    wire [15:0] overrideMemDataOut;

    mu0 mu0_inst (
        .clk(clk),
        .overrideMemControl(overrideMemControl),
        .overrideMemRnW(overrideMemRnW),
        .overrideMemAddr(overrideMemAddr),
        .overrideMemDataIn(overrideMemDataIn),
        .overrideMemDataOut(overrideMemDataOut),
        .led(led)
    );

    uart uart (
        .clk(clk),
        .uart_rx(uart_rx),
        .uart_tx(uart_tx),
        .overrideMemControl(overrideMemControl),
        .overrideMemRnW(overrideMemRnW),
        .overrideMemAddr(overrideMemAddr),
        .overrideMemDataIn(overrideMemDataIn),
        .overrideMemDataOut(overrideMemDataOut)
    );

endmodule
