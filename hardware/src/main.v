module main (
    input clk,
    input uart_rx,
    output uart_tx,
    output reg [5:0] led,
    input btn1
);

    assign led = btn1 ? 6'b111111 : 6'b000000;

    uart uart (
        .clk(clk),
        .uart_rx(uart_rx),
        .uart_tx(uart_tx)
    );

endmodule
