module main (
    input clk,
    input uart_rx,
    output uart_tx,
    output reg [5:0] led,
    input btn1
);

    assign led = btn1 ? 6'b111111 : 6'b000000;
    wire [511:0] memory;

    mu0 mu0_inst (
        .clk(clk),
        .memory(memory)
    );

    uart uart (
        .clk(clk),
        .uart_rx(uart_rx),
        .uart_tx(uart_tx),
        .memory(memory)
    );

endmodule
