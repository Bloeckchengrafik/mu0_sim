`default_nettype none


/**
 * @brief The comm module is used to control the uart communication and
 * internal execution state machine.
 *
 * The protocol is defined as follows:
 * First, transmit a command:
 * - p: Ping, just sends back a pong (P)
 * - r: Read memory, sends back the full memory values
 * - w: Write memory, writes the value specified in the next 512 bytes to the ram
 * - x: Execute code, start at 0
 * - s: Get the status of the execution state machine (+/-)
 */
module comm (
    input clk,
    output reg byteReady,
    input [7:0] dataIn,
    input byteSending,
    output reg byteReadyOut,
    output reg [7:0] dataOut
);
    localparam COMM_STATE_IDLE = 0;
    localparam COMM_STATE_ECHO = 1;
    localparam COMM_STATE_RXMEM = 2;
    localparam COMM_STATE_EXSTART = 3;
    localparam COMM_STATE_EXSTATE = 4;
    localparam COMM_STATE_TXMEM = 5;

    initial begin
        byteReadyOut = 0;
        dataOut = 0;
    end

    reg [3:0] comm_state = COMM_STATE_IDLE;
    reg byteReceived = 0;
    event useByte;

    always @(posedge clk) begin
        if (byteReady && !byteSending && !byteReceived) begin
            byteReceived <= 1'b1;
            ->useByte;
        end

        if (byteSending) begin
            byteReadyOut <= 1'b0;
            byteReceived <= 1'b0;
        end
    end

    always @(useByte) begin
        byteReadyOut <= 1'b1;
        dataOut <= dataIn;
    end

endmodule
